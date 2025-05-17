use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub teloxide_token: String,
    pub openai_api_key: Option<String>,
    pub database_url: String,
    pub sui_rpc_url: String,
    pub sui_payments_package_id: String,
    pub sui_admin_object_id: String,
    pub sui_fees_id: String,
    pub sui_relations_id: String,
    pub sui_admin_mnemonic_path: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        let teloxide_token = env::var("TELOXIDE_TOKEN").expect("TELOXIDE_TOKEN must be set");
        let openai_api_key = env::var("OPENAI_API_KEY").ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let sui_rpc_url = env::var("SUI_RPC_URL").expect("SUI_RPC_URL must be set");
        let sui_payments_package_id = env::var("SUI_PAYMENTS_PACKAGE_ID").expect("SUI_PAYMENTS_PACKAGE_ID must be set");
        let sui_admin_object_id = env::var("SUI_ADMIN_OBJECT_ID").expect("SUI_ADMIN_OBJECT_ID must be set");
        let sui_fees_id = env::var("SUI_FEES_ID").expect("SUI_FEES_ID must be set");
        let sui_relations_id = env::var("SUI_RELATIONS_ID").expect("SUI_RELATIONS_ID must be set");
        let sui_admin_mnemonic_path = env::var("SUI_ADMIN_MNEMONIC_PATH").ok();

        Config {
            teloxide_token,
            openai_api_key,
            database_url,
            sui_rpc_url,
            sui_payments_package_id,
            sui_admin_object_id,
            sui_fees_id,
            sui_relations_id,
            sui_admin_mnemonic_path,
        }
    }

    pub fn openai_api_key(&self) -> Option<String> {
        self.openai_api_key.clone()
    }
} 