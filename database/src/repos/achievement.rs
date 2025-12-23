use sqlx::{SqlitePool, query, query_as};

use crate::{
    error::DatabaseError,
    models::achievement::{Achievement, AchievementCreate, AchievementGoal},
};

pub struct AchievementRepo<'a> {
    db: &'a SqlitePool,
}

impl<'a> AchievementRepo<'a> {
    pub fn new(db: &'a SqlitePool) -> Self {
        Self { db }
    }

    async fn by_id(&self, id: u32) -> Result<Vec<AchievementGoal>, DatabaseError> {
        Ok(query_as(
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
        .await?)
    }

    async fn by_goal_id(&self, goal_id: u32) -> Result<Vec<AchievementGoal>, DatabaseError> {
        Ok(query_as(
            "
            SELECT
                achievement.id    as achievement_id,
                achievement.name  as achievement_name,
                service_id,
                goal2.id          as goal_id,
                goal2.description as goal_description,
                goal2.sequence    as goal_sequence

            FROM
                goal as goal1
                    inner join achievement on achievement.id = goal1.achievement_id
                    inner join goal as goal2 on goal2.achievement_id = achievement.id
            WHERE
                goal1.id = ?;
            ",
        )
        .bind(goal_id)
        .fetch_all(self.db)
        .await?)
    }

    pub async fn for_service(
        &self,
        service_id: u32,
    ) -> Result<Vec<AchievementGoal>, DatabaseError> {
        Ok(query_as(
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
        .await?)
    }

    /// create an achievement for a service
    ///
    /// returns the achievement with all its goals in rows
    pub async fn create_for_service(
        &self,
        service_id: u32,
        achievement: AchievementCreate,
    ) -> Result<Vec<AchievementGoal>, DatabaseError> {
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

    pub async fn unlock_goal(
        &self,
        user_id: u32,
        goal_id: u32,
    ) -> Result<Vec<AchievementGoal>, DatabaseError> {
        query(
            "
            INSERT INTO
                unlock (user_id, goal_id)
            VALUES
                (?,?);
            ",
        )
        .bind(user_id)
        .bind(goal_id)
        .execute(self.db)
        .await?;

        self.by_goal_id(goal_id).await
    }
}
