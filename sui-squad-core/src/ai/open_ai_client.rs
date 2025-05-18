use openai_api_rs::v1::api::OpenAIClient as ApiClient;
use openai_api_rs::v1::chat_completion::{ChatCompletionRequest, ChatCompletionMessage, MessageRole, Content};
use openai_api_rs::v1::common::GPT4_O;
use crate::config::Config;
use crate::error::CoreError;

/// Client for OpenAI-based chat completions.
#[derive(Debug)]
pub struct OpenAiClient {
    client: ApiClient,
}

impl OpenAiClient {
    /// Create a new OpenAiClient from configuration.
    pub fn new(config: &Config) -> Result<Self, CoreError> {
        let api_key = config.openai_api_key()
            .ok_or_else(|| CoreError::ConfigurationError("OpenAI API key not found".to_string()))?;
        let client = ApiClient::builder()
            .with_api_key(api_key)
            .build()
            .map_err(|e| CoreError::OpenAiOtherError(format!("OpenAI client build failed: {}", e)))?;
        Ok(OpenAiClient { client })
    }

    /// Send a chat completion request and return the assistant's response text.
    pub async fn generate_response(&mut self, user_input: &str) -> Result<String, CoreError> {
        // System prompt to define assistant role
        let system_message = ChatCompletionMessage {
            role: MessageRole::user,
            content: Content::Text("You are a helpful assistant for the Sui Squad Telegram bot. Always answer as clearly and concisely as possible.".to_string()),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        };
        // User message
        let user_message = ChatCompletionMessage {
            role: MessageRole::user,
            content: Content::Text(user_input.to_string()),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        };
        let req = ChatCompletionRequest::new(
            GPT4_O.to_string(),
            vec![system_message, user_message],
        );
        let res = self.client.chat_completion(req)
            .await
            .map_err(|e| CoreError::OpenAiOtherError(format!("OpenAI chat failed: {}", e)))?;
        // Extract first choice content
        let choice = res.choices
            .get(0)
            .ok_or_else(|| CoreError::OpenAiOtherError("No choices in response".to_string()))?;
        match &choice.message.content {
            Some(text) => Ok(text.clone()),
            None => Err(CoreError::OpenAiOtherError("No content in response message".to_string())),
        }
    }
} 