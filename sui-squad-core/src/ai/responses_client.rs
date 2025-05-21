use crate::config::Config;
use crate::error::CoreError;
use openai_responses::Client as OAIClient;
use openai_responses::types::{Input, Model, Request, Response as OAIResponse, Tool, ToolChoice};
use tracing::{debug, error, info};

/// Client for OpenAI-based responses using openai_responses SDK.
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

        debug!("Initializing OpenAI Responses client");
        let client = OAIClient::new(&api_key).map_err(|e| {
            CoreError::ConfigurationError(format!(
                "Failed to create OpenAI Responses client: {}",
                e
            ))
        })?;

        info!("OpenAI Responses client initialized successfully");
        Ok(ResponsesClient { client })
    }

    /// Generates a text response for the given user input.
    pub async fn generate_response(&self, user_input: &str) -> Result<String, CoreError> {
        debug!("Generating response for user input: {}", user_input);
        let mut request = Request::default();
        request.model = Model::GPT4o;
        request.instructions = Some("You are a helpful assistant.".to_string());
        request.input = Input::Text(user_input.to_string());

        debug!("Sending request to OpenAI API");
        let result = match self.client.create(request).await {
            Ok(result) => result,
            Err(e) => {
                error!("Network error when calling OpenAI API: {}", e);
                return Err(CoreError::Other(format!("Network error: {}", e)));
            }
        };

        match result {
            Ok(resp) => {
                debug!("Successfully received response from OpenAI");
                Ok(resp.output_text())
            }
            Err(api_err) => {
                error!("OpenAI API error: {:?}", api_err);
                Err(CoreError::Other(format!("API error: {:?}", api_err)))
            }
        }
    }

    /// Generates a response allowing the model to call specified custom tools based on JSON schema.
    pub async fn generate_with_tools(
        &self,
        user_input: &str,
        tools: Vec<Tool>,
    ) -> Result<OAIResponse, CoreError> {
        debug!(
            "Generating response with tools for user input: {}",
            user_input
        );
        let mut request = Request::default();
        request.model = Model::GPT4o;
        request.instructions = Some("You are a helpful assistant.".to_string());
        request.input = Input::Text(user_input.to_string());
        request.tools = Some(tools);
        request.tool_choice = Some(ToolChoice::Auto);

        debug!("Sending request with tools to OpenAI API");
        let result = match self.client.create(request).await {
            Ok(result) => result,
            Err(e) => {
                error!("Network error when calling OpenAI API with tools: {}", e);
                return Err(CoreError::Other(format!("Network error: {}", e)));
            }
        };

        match result {
            Ok(resp) => {
                debug!("Successfully received response with tools from OpenAI");
                Ok(resp)
            }
            Err(api_err) => {
                error!("OpenAI API error in tools request: {:?}", api_err);
                Err(CoreError::Other(format!("API error: {:?}", api_err)))
            }
        }
    }
}
