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
