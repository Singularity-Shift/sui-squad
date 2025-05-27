use crate::config::Config;
use crate::error::CoreError;
use open_ai_rust_responses_by_sshift::{Client as OAIClient, Request, Model};
use open_ai_rust_responses_by_sshift::types::{
    Tool, Response as OAIResponse,
};

/// Client for OpenAI-based responses using open_ai_rust_responses_by_sshift SDK.
#[derive(Clone)]
pub struct ResponsesClient {
    client: OAIClient,
}

impl ResponsesClient {
    /// Creates a new ResponsesClient with the given Config.
    pub fn new(config: &Config) -> Result<Self, CoreError> {
        let api_key = config
            .openai_api_key()
            .ok_or_else(|| CoreError::ConfigurationError("OpenAI API key not found".to_string()))?;
        let client = OAIClient::new(&api_key).map_err(|e| {
            CoreError::ConfigurationError(format!(
                "Failed to create OpenAI Responses client: {}",
                e
            ))
        })?;
        Ok(ResponsesClient { client })
    }

    /// Generates a text response for the given user input.
    pub async fn generate_response(&self, user_input: &str) -> Result<OAIResponse, CoreError> {
        let request = Request::builder()
            .model(Model::GPT4oMini)
            .input(user_input)
            .instructions("You are a helpful assistant.")
            .build();

        let response = self
            .client
            .responses
            .create(request)
            .await
            .map_err(|e| CoreError::Other(e.to_string()))?;

        Ok(response)
    }

    /// Generates a response allowing the model to call specified custom tools based on JSON schema.
    pub async fn generate_with_tools(
        &self,
        user_input: &str,
        tools: Vec<Tool>,
    ) -> Result<OAIResponse, CoreError> {
        let request = Request::builder()
            .model(Model::GPT4oMini)  // Using GPT-4o Mini for efficient processing with tools
            .input(user_input)
            .instructions("You are a helpful assistant.")
            .tools(tools)
            .build();

        let response = self
            .client
            .responses
            .create(request)
            .await
            .map_err(|e| CoreError::Other(e.to_string()))?;

        println!("result {:?}", response);

        Ok(response)
    }
}
