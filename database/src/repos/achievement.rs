use std::iter::Peekable;

use sqlx::{SqlitePool, query, query_as};

use crate::{
    error::DatabaseError,
    models::achievement::{
        Achievement, AchievementCreatePayload, AchievementGoal, AchievementPayload, GoalPayload,
    },
};

pub struct AchievementRepo<'a> {
    db: &'a SqlitePool,
}

impl<'a> AchievementRepo<'a> {
    pub fn new(db: &'a SqlitePool) -> Self {
        Self { db }
    }

    pub async fn for_service(
        &self,
        service_id: u32,
    ) -> Result<Vec<AchievementPayload>, DatabaseError> {
        let rows: Vec<AchievementGoal> = query_as(
            "SELECT
                achievement.id as achievement_id,
                name as achievement_name,
                service_id,
                goal.id as goal_id,
                description as goal_description,
                sequence as goal_sequence
            FROM
                achievement
            INNER JOIN
                goal
                ON goal.achievement_id = achievement.id
            WHERE
                service_id = ?
            ORDER BY
                achievement_id, goal_sequence
            ;
            ",
        )
        .bind(service_id)
        .fetch_all(self.db)
        .await?;

        let mut rows = rows.into_iter().peekable();

        let mut achievements = Vec::new();
        while let Some(achievement) = Self::unpack_next_achievement(&mut rows) {
            achievements.push(achievement);
        }

        Ok(achievements)
    }

    async fn by_id(&self, id: u32) -> Result<AchievementPayload, DatabaseError> {
        let rows: Vec<AchievementGoal> = query_as(
            "SELECT
                achievement.id as achievement_id,
                name as achievement_name,
                service_id,
                goal.id as goal_id,
                description as goal_description,
                sequence as goal_sequence
            FROM
                achievement
            INNER JOIN
                goal
                ON goal.achievement_id = achievement.id
            WHERE
                achievement.id = ?
            ORDER BY
                achievement_id, goal_sequence
            ;
            ",
        )
        .bind(id)
        .fetch_all(self.db)
        .await?;

        let mut rows = rows.into_iter().peekable();

        let achievement =
            Self::unpack_next_achievement(&mut rows).ok_or(DatabaseError::NotFound)?;
        Ok(achievement)
    }

    pub async fn create_for_service(
        &self,
        service_id: u32,
        achievement: AchievementCreatePayload,
    ) -> Result<AchievementPayload, DatabaseError> {
        let mut tx = self.db.begin().await?;

        // insert the achievement
        let db_achievement: Achievement = query_as(
            "
            INSERT INTO
                achievement
                (name, service_id)
            VALUES
                (?, ?)
            RETURNING
                id, name, service_id
            ;
            ",
        )
        .bind(achievement.name)
        .bind(service_id)
        .fetch_one(&mut *tx)
        .await?;

        // attach all goals to the achievement
        for goal in achievement.goals {
            query(
                "
                INSERT INTO
                    goal
                    (description, achievement_id, sequence)
                VALUES
                    (?, ?, ?)
                ;
                ",
            )
            .bind(goal.description)
            .bind(db_achievement.id)
            .bind(goal.sequence)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        self.by_id(db_achievement.id).await
    }

    fn unpack_next_achievement<I>(rows: &mut Peekable<I>) -> Option<AchievementPayload>
    where
        I: Iterator<Item = AchievementGoal>,
    {
        // get first row
        let row = rows.next()?;

        // make a new achievement with the first goal
        let mut achievement = AchievementPayload {
            id: row.achievement_id,
            name: row.achievement_name,
            goals: vec![GoalPayload {
                id: row.goal_id,
                description: row.goal_description,
                sequence: row.goal_sequence,
            }],
        };

        // add all following goals for the same achievement
        while let Some(next_row) = rows.peek() {
            if next_row.achievement_id != achievement.id {
                break;
            }

            if let Some(next_goal) = rows.next() {
                achievement.goals.push(GoalPayload {
                    id: next_goal.goal_id,
                    description: next_goal.goal_description,
                    sequence: next_goal.goal_sequence,
                });
            }
        }

        Some(achievement)
    }
}
