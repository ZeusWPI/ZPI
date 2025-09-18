use database::models::achievement::GoalCreate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GoalPayload {
    pub id: i32,
    pub description: String,
    pub sequence: i32,
}

#[derive(Serialize, Deserialize)]
pub struct GoalCreatePayload {
    pub description: String,
    pub sequence: u32,
}

impl From<GoalCreatePayload> for GoalCreate {
    fn from(value: GoalCreatePayload) -> Self {
        GoalCreate {
            description: value.description,
            sequence: value.sequence,
        }
    }
}
