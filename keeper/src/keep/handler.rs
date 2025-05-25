use std::sync::Arc;

use axum::{extract::State, response::Json};

use crate::state::KeeperState;

use super::dto::{AuthResponse, JwtPayload, User};

#[utoipa::path(
    post,
    path = "/keep",
    summary = "Get information about the Keeper service",
    description = "Saves jwt token for the specified ID",
    request_body = [JwtPayload],
    responses(
        (status = 201, description = "HTML Page", body = [AuthResponse])
    )
)]
#[axum::debug_handler]
pub async fn keep(State(keeper_state): State<Arc<KeeperState>>, Json(jwt_payload): Json<JwtPayload>) -> Json<AuthResponse> {
    let db = keeper_state.db();
    let mut squard_connect_client = keeper_state.squard_connect_client().clone();

    squard_connect_client.set_jwt(jwt_payload.token.clone());

    let account = squard_connect_client.get_address().await.unwrap();

    // Save user data to database
    let key = format!("user:{}", jwt_payload.username);

    println!("account: {}", account.address);

    let user = User::from((jwt_payload.token, account.address));


    let value = serde_json::to_string(&user).unwrap_or_default();
    
    match db.insert(key, value.as_bytes()) {
        Ok(_) => Json(AuthResponse { success: true }),
        Err(_) => Json(AuthResponse { success: false }),
    }
}
