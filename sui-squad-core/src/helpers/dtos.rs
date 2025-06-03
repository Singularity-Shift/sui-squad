use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct User {
    pub jwt: String,
    pub bot_id: String,
    pub network: String,
    pub public_key: String,
    pub max_epoch: u64,
    pub randomness: String,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct AuthRequest {
    pub bot_id: String,
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserPayload {
    pub telegram_id: String,
    pub group_telegram_id: String,
    pub wallet: String,
}

impl From<JwtPayload> for User {
    fn from(jwt_payload: JwtPayload) -> Self {
        Self {
            jwt: jwt_payload.token,
            bot_id: jwt_payload.bot_id,
            network: jwt_payload.network,
            public_key: jwt_payload.public_key,
            max_epoch: jwt_payload.max_epoch,
            randomness: jwt_payload.randomness,
        }
    }
}
