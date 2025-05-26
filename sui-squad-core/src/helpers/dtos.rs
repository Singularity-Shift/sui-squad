use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct JwtPayload {
    pub token: String,
    pub username: String,
    pub chat_id: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct User {
    pub jwt: String,
    pub address: String,
    pub chat_id: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct AuthRequest {
    pub chat_id: String,
    pub username: String,
}

impl From<(JwtPayload, String)> for User  {
    fn from(state: (JwtPayload, String)) -> Self {
        let (jwt, address) = state;

        Self { jwt: jwt.token, address: address, chat_id: jwt.chat_id }
    }
}
