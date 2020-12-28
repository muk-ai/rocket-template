use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub completed: bool,
}
