use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// Use our own UserId type instead of teloxide's
pub type UserId = i64;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct JwtPayload {
    pub token: String,
    pub user_id: String,
    pub bot_id: String,
    pub network: String,
    pub public_key: String,
    pub max_epoch: u64,
    pub randomness: String,
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub storage: Storage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Storage {
    pub jwt: String,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct FundRequest {
    pub bot_id: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserPayload {
    pub telegram_id: String,
}
