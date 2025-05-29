use std::sync::Arc;

use axum::{extract::State, response::{Json, Result}};

use crate::{error::ErrorKeeper, state::KeeperState};

use sui_squad_core::helpers::dtos::{AuthRequest, JwtPayload, User};
use super::dto::AuthResponse;

#[utoipa::path(
    post,
    path = "/keep",
    summary = "Get information about the Keeper service",
    description = "Saves jwt token for the specified ID",
    request_body = [JwtPayload],
    responses(
        (status = 201, description = "Response if user is saved", body = [AuthResponse])
    )
)]
#[axum::debug_handler]
pub async fn keep(State(keeper_state): State<Arc<KeeperState>>, Json(jwt_payload): Json<JwtPayload>) -> Json<AuthResponse> {
    let db = keeper_state.db();
    let mut squard_connect_client = keeper_state.squard_connect_client().clone();

    squard_connect_client.set_jwt(jwt_payload.token.clone());

    // Save user data to database
    let key = format!("user:{}", jwt_payload.user_id);

    let user = User::from(jwt_payload);


    let value = serde_json::to_string(&user).unwrap_or_default();
    
    match db.insert(key, value.as_bytes()) {
        Ok(_) => Json(AuthResponse { success: true }),
        Err(_) => Json(AuthResponse { success: false }),
    }
}

#[utoipa::path(
    post,
    path = "/auth",
    summary = "Get user auth info",
    description = "Get user auth info",
    request_body = [AuthRequest],
    responses(
        (status = 201, description = "Get user auth info", body = [User])
    )
)]
#[axum::debug_handler]
pub async fn auth(State(keeper_state): State<Arc<KeeperState>>, Json(auth_request): Json<AuthRequest>) -> Result<Json<User>, ErrorKeeper> {
    let db = keeper_state.db();

    let key = format!("user:{}", auth_request.user_id);

    let value = db.get(key)
        .map_err(|e| ErrorKeeper { message: e.to_string(), status: 500 })?
        .ok_or_else(|| ErrorKeeper { message: "User not found".to_string(), status: 404 })?;

    let user: User = serde_json::from_slice(&value)
        .map_err(|e| ErrorKeeper { message: e.to_string(), status: 500 })?;

    if user.bot_id != auth_request.bot_id {
        return Err(ErrorKeeper { message: "Invalid bot ID".to_string(), status: 401 });
    }

    Ok(Json(user))
}
