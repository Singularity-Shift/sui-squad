use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct FundRequest {
    pub public_key: String,
    pub max_epoch: u64,
    pub telegram_id: String,
    pub randomness: String,
}
