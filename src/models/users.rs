use serde::{Deserialize, Serialize};

mod from_request;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub uid: String,
}
