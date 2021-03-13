use serde::{Deserialize, Serialize};

mod from_request;
pub mod repository;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub uid: String,
}
