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
    pub chat_id: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct AuthRequest {
    pub chat_id: String,
    pub username: String,
}

impl From<JwtPayload> for User  {
    fn from(jwt_payload: JwtPayload) -> Self {

        Self { jwt: jwt_payload.token, chat_id: jwt_payload.chat_id }
    }
}
