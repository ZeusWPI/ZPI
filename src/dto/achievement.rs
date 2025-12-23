use std::iter::Peekable;

use database::{
    Database,
    models::achievement::{AchievementCreate, AchievementGoal},
};
use serde::{Deserialize, Serialize};

use crate::{
    dto::goal::{GoalCreatePayload, GoalPayload, GoalUnlockedPayload},
    error::AppError,
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AchievementPayload {
    pub id: i32,
    pub name: String,
    pub goals: Vec<GoalPayload>,
}

impl AchievementPayload {
    pub async fn for_service(
        db: &Database,
        service_id: u32,
    ) -> Result<Vec<AchievementPayload>, AppError> {
        let rows = db.achievements().for_service(service_id).await?;

        let mut rows = rows.into_iter().peekable();

        let mut achievements = Vec::new();
        while let Some(achievement) = unpack_next_achievement(&mut rows) {
            achievements.push(achievement);
        }

        Ok(achievements)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AchievementUnlockedPayload {
    pub id: i32,
    pub name: String,
    pub goals: Vec<GoalUnlockedPayload>,
}

impl AchievementUnlockedPayload {
    // TODO unlock goal
}

#[derive(Serialize, Deserialize)]
pub struct AchievementCreatePayload {
    pub name: String,
    pub goals: Vec<GoalCreatePayload>,
}

impl AchievementCreatePayload {
    pub async fn create(
        mut self,
        service_id: u32,
        db: &Database,
    ) -> Result<AchievementPayload, AppError> {
        if self.goals.is_empty() {
            return Err(AppError::PayloadError("Expected at least one goal".into()));
        }

        self.goals.sort_by_key(|x| x.sequence);
        let ordered_1_seperated = self
            .goals
            .iter()
            .map(|x| x.sequence)
            .collect::<Vec<u32>>()
            .windows(2)
            .all(|w| match (w.first(), w.get(1)) {
                (Some(first), Some(second)) => second - first == 1,
                _ => false,
            });
        if let Some(goal) = self.goals.first()
            && (goal.sequence != 0 || !ordered_1_seperated)
        {
            return Err(AppError::PayloadError(
                "Sequence should start with 0 and count up by 1".into(),
            ));
        }

        let rows = db
            .achievements()
            .create_for_service(
                service_id,
                AchievementCreate {
                    name: self.name,
                    goals: self.goals.into_iter().map(|x| x.into()).collect(),
                },
            )
            .await?;

        // pack rows into an achievement payload
        let mut rows = rows.into_iter().peekable();
        let achievement = unpack_next_achievement(&mut rows).ok_or(AppError::NotFound)?;
        Ok(achievement)
    }
}

/// unpacks an achievement from database rows into a payload
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
