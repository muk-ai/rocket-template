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
    pub fn from_task(task: TaskDescriptionData, user_id: uuid::Uuid) -> InsertableTask {
        InsertableTask {
            description: task.description,
            completed: false,
            user_id,
        }
    }
}

#[derive(Deserialize)]
pub struct TaskDescriptionData {
    description: String,
}
