use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct FundRequest {
    pub telegram_id: String,
    pub amount: u64,
}
