use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub bot_token: String,
    pub api_id: i32,
    pub api_hash: String,
    pub openai_api_key: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        let bot_token = env::var("BOT_TOKEN").expect("BOT_TOKEN must be set");
        let api_id = env::var("API_ID")
            .expect("API_ID must be set")
            .parse::<i32>()
            .expect("API_ID must be a valid integer");
        let api_hash = env::var("API_HASH").expect("API_HASH must be set");
        let openai_api_key = env::var("OPENAI_API_KEY").ok();
        Config {
            bot_token,
            api_id,
            api_hash,
            openai_api_key,
        }
    }

    pub fn openai_api_key(&self) -> Option<String> {
        self.openai_api_key.clone()
    }
}
