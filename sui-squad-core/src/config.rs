use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub teloxide_token: String,
    pub openai_api_key: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        let teloxide_token = env::var("TELOXIDE_TOKEN").expect("TELOXIDE_TOKEN must be set");
        let openai_api_key = env::var("OPENAI_API_KEY").ok();
        Config {
            teloxide_token,
            openai_api_key,
        }
    }

    pub fn openai_api_key(&self) -> Option<String> {
        self.openai_api_key.clone()
    }
}
