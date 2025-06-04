use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub telegram_id: String,
    pub max_epoch: u64,
    pub public_key: String,
}

impl From<(String, u64, String)> for State {
    fn from(state: (String, u64, String)) -> Self {
        let (telegram_id, max_epoch, public_key) = state;

        Self {
            telegram_id,
            max_epoch,
            public_key,
        }
    }
}
