use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub completed: bool,
    pub created_at: chrono::NaiveDateTime,
    pub user_id: uuid::Uuid,
}
