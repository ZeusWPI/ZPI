use std::iter::Peekable;

use database::{
    Database,
    models::achievement::{AchievementCreate, AchievementGoal, AchievementGoalUnlock},
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

impl From<AchievementGoal> for AchievementPayload {
    fn from(row: AchievementGoal) -> Self {
        Self {
            id: row.achievement_id,
            name: row.achievement_name,
            goals: vec![GoalPayload {
                id: row.goal_id,
                description: row.goal_description,
                sequence: row.goal_sequence,
            }],
        }
    }
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

    pub async fn unlock_goal(
        db: &Database,
        user_id: u32,
        goal_id: u32,
    ) -> Result<AchievementPayload, AppError> {
        if !db.achievements().goal_exist(goal_id).await? {
            return Err(AppError::NotFound);
        }

        // FIXME improve
        let rows = if db.achievements().goal_unlocked(goal_id).await? {
            // goal already unlocked
            db.achievements().by_goal_id(goal_id).await?
        } else {
            db.achievements().unlock_goal(user_id, goal_id).await?
        };

        // pack rows into an achievement payload
        let mut rows = rows.into_iter().peekable();
        let achievement = unpack_next_achievement(&mut rows).ok_or(AppError::NotFound)?;
        Ok(achievement)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AchievementUnlockedPayload {
    pub id: i32,
    pub name: String,
    pub goals: Vec<GoalUnlockedPayload>,
}

impl From<AchievementGoalUnlock> for AchievementUnlockedPayload {
    fn from(row: AchievementGoalUnlock) -> Self {
        Self {
            id: row.achievement_id,
            name: row.achievement_name,
            goals: vec![GoalUnlockedPayload {
                id: row.goal_id,
                description: row.goal_description,
                sequence: row.goal_sequence,
                time: row.time,
            }],
        }
    }
}

impl AchievementUnlockedPayload {
    pub async fn for_user(
        db: &Database,
        user_id: u32,
    ) -> Result<Vec<AchievementUnlockedPayload>, AppError> {
        let rows = db.achievements().unlocked_for_user(user_id).await?;

        let mut rows = rows.into_iter().peekable();

        let mut achievements = Vec::new();
        while let Some(achievement) = unpack_next_achievement(&mut rows) {
            achievements.push(achievement);
        }

        Ok(achievements)
    }
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
        let ordered_1_separated = self
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
            && (goal.sequence != 0 || !ordered_1_separated)
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

pub trait AchievementRow: Sized {
    type Payload: From<Self>;

    fn achievement_id(&self) -> i32;

    fn push_into(self, payload: &mut Self::Payload);
}

impl AchievementRow for AchievementGoal {
    type Payload = AchievementPayload;

    fn achievement_id(&self) -> i32 {
        self.achievement_id
    }

    fn push_into(self, payload: &mut Self::Payload) {
        payload.goals.push(GoalPayload {
            id: self.goal_id,
            description: self.goal_description,
            sequence: self.goal_sequence,
        });
    }
}

impl AchievementRow for AchievementGoalUnlock {
    type Payload = AchievementUnlockedPayload;

    fn achievement_id(&self) -> i32 {
        self.achievement_id
    }

    fn push_into(self, payload: &mut Self::Payload) {
        payload.goals.push(GoalUnlockedPayload {
            id: self.goal_id,
            description: self.goal_description,
            sequence: self.goal_sequence,
            time: self.time,
        });
    }
}

/// unpacks an achievement from database rows into a payload
fn unpack_next_achievement<I, R>(rows: &mut Peekable<I>) -> Option<R::Payload>
where
    I: Iterator<Item = R>,
    R: AchievementRow,
{
    // get first row
    let first_row = rows.next()?;
    let current_achievement_id = first_row.achievement_id();

    // make a new achievement with the first goal
    let mut achievement: R::Payload = first_row.into();

    // add all following goals for the same achievement
    while let Some(next_row) = rows.next_if(|x| x.achievement_id() == current_achievement_id) {
        next_row.push_into(&mut achievement);
    }

    Some(achievement)
}
