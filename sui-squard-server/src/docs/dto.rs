use crate::fund;
use crate::fund::dto::FundRequest;
use crate::info;
use crate::webhook;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(info::handler::info, webhook::handler::webhook, fund::handler::fund),
    components(schemas(info::dto::Info, FundRequest))
)]
pub struct ApiDoc;
