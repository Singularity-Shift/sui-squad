use reqwest::Client;
use serde_json::Value;

/// Enum representing parsed intents from natural language inputs.
#[derive(Debug)]
pub enum Intent {
    // Define intents, e.g., Balance, Pay, Claim, etc.
}

/// Client for OpenAI-based intent parsing.
pub struct OpenAIClient {
    client: Client,
    api_key: Option<String>,
}

impl OpenAIClient {
    pub fn new(api_key: Option<String>) -> Self {
        OpenAIClient {
            client: Client::new(),
            api_key,
        }
    }

    /// Parse user input into a bot intent.
    pub async fn parse_intent(&self, text: &str) -> Option<Intent> {
        if self.api_key.is_none() {
            return None;
        }
        // TODO: call OpenAI API and map response to Intent
        None
    }
} 