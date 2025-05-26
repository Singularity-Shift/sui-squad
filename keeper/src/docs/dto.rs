use crate::info;
use crate::keep;
use crate::keep::dto::AuthResponse;
use crate::webhook;
use utoipa::OpenApi;
use sui_squad_core::helpers::dtos::{JwtPayload, User, AuthRequest};

#[derive(OpenApi)]
#[openapi(
    paths(info::handler::info, webhook::handler::webhook, keep::handler::keep, keep::handler::auth),
    components(schemas(info::dto::Info, JwtPayload, AuthRequest, User, AuthResponse))
)]
pub struct ApiDoc;
