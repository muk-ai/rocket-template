use crate::schema::users;
use serde::{Deserialize, Serialize};

mod from_request;
pub mod repository;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
pub struct User {
    pub uid: String,
    pub created_at: chrono::NaiveDateTime,
    pub id: uuid::Uuid,
}
