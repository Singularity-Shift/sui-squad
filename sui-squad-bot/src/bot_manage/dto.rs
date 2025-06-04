use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub telegram_id: String,
    pub max_epoch: u64,
    pub public_key: String,
    pub randomness: String,
}

impl From<(String, u64, String, String)> for State {
    fn from(state: (String, u64, String, String)) -> Self {
        let (telegram_id, max_epoch, public_key, randomness) = state;

        Self {
            telegram_id,
            max_epoch,
            public_key,
            randomness,
        }
    }
}
