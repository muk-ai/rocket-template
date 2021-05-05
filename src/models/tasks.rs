use crate::models::users::User;
use crate::schema::tasks;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable)]
#[belongs_to(User)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub completed: bool,
    pub created_at: chrono::NaiveDateTime,
    pub user_id: uuid::Uuid,
}

#[derive(Insertable)]
#[table_name = "tasks"]
pub struct InsertableTask {
    description: String,
    completed: bool,
    user_id: uuid::Uuid,
}

impl InsertableTask {
    pub fn build(description: String, user_id: uuid::Uuid) -> InsertableTask {
        InsertableTask {
            description,
            user_id,
            completed: false,
        }
    }
}
