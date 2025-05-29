use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub user_id: String,
    pub bot_id: String,
    pub network: String,
    pub public_key: String,
    pub max_epoch: u64,
    pub randomness: String,
}

impl From<(String, String, String, String, u64, String)> for State {
    fn from(state: (String, String, String, String, u64, String)) -> Self {
        let (user_id, bot_id, network, public_key, max_epoch, randomness) = state;

        Self { user_id, bot_id, network, public_key, max_epoch, randomness }
    }
}