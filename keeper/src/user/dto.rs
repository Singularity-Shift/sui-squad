#

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct User {
    pub telegram_id: u64,
    pub group_telegram_id: u64,
    pub wallet: String,
}