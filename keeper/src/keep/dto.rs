use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct JwtPayload {
    pub token: String,
    pub username: String,
}

#[derive(Serialize, ToSchema)]
pub struct AuthResponse {
    pub success: bool,
}

#[derive(Serialize)]
pub struct User {
    jwt: String,
    address: String,
}

impl From<(String, String)> for User  {
    fn from(state: (String, String)) -> Self {
        let (jwt, address) = state;

        Self { jwt, address }
    }
}

