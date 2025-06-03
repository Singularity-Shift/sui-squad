use serde::Serialize;
use utoipa::ToResponse;
use axum::{response::{IntoResponse, Response}, http::StatusCode, Json};

#[derive(Debug, Serialize, ToResponse)]
pub struct ErrorKeeper {
    pub message: String,
    pub status: u16,
}

impl IntoResponse for ErrorKeeper {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self)).into_response()
    }
}