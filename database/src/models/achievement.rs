use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct Goal {
    pub id: u32,
    pub description: String,
    pub achievement_id: u32,
    pub sequence: u32,
}

#[derive(Debug, FromRow)]
pub struct Achievement {
    pub id: u32,
    pub name: String,
    pub service_id: u32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AchievementGoal {
    pub achievement_id: i32,
    pub achievement_name: String,
    pub service_id: i32,
    pub goal_id: i32,
    pub goal_description: String,
    pub goal_sequence: i32,
}

#[derive(Serialize, Deserialize)]
pub struct GoalCreate {
    pub description: String,
    pub sequence: u32,
}

#[derive(Serialize, Deserialize)]
pub struct AchievementCreate {
    pub name: String,
    pub goals: Vec<GoalCreate>,
}
