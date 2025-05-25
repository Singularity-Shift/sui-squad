use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub username: String,
}

impl From<String> for State {
    fn from(username: String) -> Self {
        Self { username }
    }
}