use crate::info;
use crate::keep;
use crate::webhook;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(info::handler::info, webhook::handler::webhook, keep::handler::keep),
    components(schemas(info::dto::Info, keep::dto::JwtPayload))
)]
pub struct ApiDoc;
