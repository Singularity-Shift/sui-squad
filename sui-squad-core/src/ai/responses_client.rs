use crate::config::Config;
use crate::error::CoreError;
use openai_responses::Client as OAIClient;
use openai_responses::types::{
    Error, Input, Model, Request, Response as OAIResponse, ResponseResult, Tool, ToolChoice,
};

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
        let mut request = Request::default();
        request.model = Model::GPT4o;
        request.instructions = Some("You are a helpful assistant.".to_string());
        request.input = Input::Text(user_input.to_string());
        let result = self
            .client
            .create(request)
            .await
            .map_err(|e| CoreError::Other(e.to_string()))?;

        let response: Result<Result<OAIResponse, Error>, reqwest::Error> =
            result.json::<ResponseResult>().await.map(Into::into);

        match response {
            Ok(resp) => Ok(resp.unwrap()),
            Err(api_err) => Err(CoreError::Other(format!("{:?}", api_err))),
        }
    }

    /// Generates a response allowing the model to call specified custom tools based on JSON schema.
    pub async fn generate_with_tools(
        &self,
        user_input: &str,
        tools: Vec<Tool>,
    ) -> Result<OAIResponse, CoreError> {
        let mut request = Request::default();
        request.model = Model::Other(String::from("gpt-4.1-nano"));
        request.instructions = Some("You are a helpful assistant.".to_string());
        request.input = Input::Text(user_input.to_string());
        request.tools = Some(tools);
        request.tool_choice = Some(ToolChoice::Auto);
        let result = self
            .client
            .create(request)
            .await
            .map_err(|e| CoreError::Other(e.to_string()))?;

        println!("result {:?}", result);

        let response: OAIResponse = result
            .json::<OAIResponse>()
            .await
            .map_err(|e| CoreError::Other(e.to_string()))?;

        Ok(response)
    }
}
