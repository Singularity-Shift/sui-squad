use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub teloxide_token: String,
    pub openai_api_key: Option<String>,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        let teloxide_token = env::var("TELOXIDE_TOKEN").expect("TELOXIDE_TOKEN must be set");
        let openai_api_key = env::var("OPENAI_API_KEY").ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Config {
            teloxide_token,
            openai_api_key,
            database_url,
        }
    }
} 