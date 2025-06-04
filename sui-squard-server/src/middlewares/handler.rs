use axum::{extract::Request, http::HeaderMap, middleware::Next, response::Response};
use sui_squad_core::helpers::{
    dtos::{User, UserPayload},
    jwt::JwtManager,
};

use crate::error::ErrorKeeper;

pub async fn auth(mut req: Request, next: Next) -> Result<Response, ErrorKeeper> {
    let headers = req.headers();
    let token = headers.get("Authorization").and_then(|h| h.to_str().ok());

    if let Some(token) = token {
        let jwt_manager = JwtManager::new();
        let token = token.replace("Bearer ", "");
        let claims = jwt_manager
            .validate_token(&token)
            .map_err(|e| ErrorKeeper {
                message: e.to_string(),
                status: 401,
            })?;

        let telegram_id = claims.telegram_id;

        let user = UserPayload {
            telegram_id: telegram_id.to_string(),
        };

        req.extensions_mut().insert(user);
    } else {
        return Err(ErrorKeeper {
            message: "Unauthorized".to_string(),
            status: 401,
        });
    }

    Ok(next.run(req).await)
}
