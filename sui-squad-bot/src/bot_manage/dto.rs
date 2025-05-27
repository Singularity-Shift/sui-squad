use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub username: String,
    pub chat_id: String,
}

impl From<(String, String)> for State {
    fn from(state: (String, String)) -> Self {
        let (username, chat_id) = state;

        Self { username, chat_id }
    }
}