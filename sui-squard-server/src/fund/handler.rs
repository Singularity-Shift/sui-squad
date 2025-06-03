use std::sync::Arc;

use axum::{
    extract::State,
    http::HeaderMap,
    response::{Json, Result},
};

use crate::{error::ErrorKeeper, state::KeeperState};

use super::dto::FundRequest;
use sui_squad_core::helpers::dtos::User;

#[utoipa::path(
    post,
    path = "/fund",
    summary = "Get user auth info",
    description = "Get user auth info",
    request_body = [FundRequest],
    responses(
        (status = 201, description = "Get user auth info", body = [User])
    )
)]
#[axum::debug_handler]
pub async fn fund(
    State(keeper_state): State<Arc<KeeperState>>,
    headers: HeaderMap,
    Json(auth_request): Json<FundRequest>,
) -> Result<(), ErrorKeeper> {
}
