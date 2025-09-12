use sqlx::{SqlitePool, query_as};

use crate::{
    error::DatabaseError,
    models::achievement::{AchievementGoal, AchievementPayload, GoalPayload},
};

pub struct AchievementRepo<'a> {
    db: &'a SqlitePool,
}

impl<'a> AchievementRepo<'a> {
    pub fn new(db: &'a SqlitePool) -> Self {
        Self { db }
    }

    async fn goals_for_service(
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

    pub async fn for_service(
        &self,
        service_id: u32,
    ) -> Result<Vec<AchievementPayload>, DatabaseError> {
        let mut rows = self
            .goals_for_service(service_id)
            .await?
            .into_iter()
            .peekable();

        let mut achievements = Vec::new();
        while let Some(row) = rows.next() {
            // make a new achievement
            let mut achievement = AchievementPayload {
                id: row.achievement_id,
                name: row.achievement_name,
                goals: vec![GoalPayload {
                    id: row.goal_id,
                    description: row.goal_description,
                    sequence: row.goal_sequence,
                }],
            };

            // as long as the achievement is the same, add goals to the achievement
            while let Some(next_row) = rows.peek() {
                if next_row.achievement_id != row.achievement_id {
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

            achievements.push(achievement);
        }

        Ok(achievements)
    }
}
