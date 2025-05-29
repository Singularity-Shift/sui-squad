use crate::config::Config;
use crate::error::CoreError;
use open_ai_rust_responses_by_sshift::{Client as OAIClient, Request, Model};
use open_ai_rust_responses_by_sshift::types::{
    Tool, Response as OAIResponse, ToolChoice,
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
            .model(Model::GPT41Mini)
            .input(user_input)
            .instructions("You are SUI Squad Bot, a Sui blockchain wallet assistant for Telegram groups. Respond conversationally and provide helpful information about wallet functionality. For specific wallet actions like checking balance, getting address, sending, or withdrawing, let users know they can use the specific wallet commands.")
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
        let instructions = "You are SUI Squad Bot, a Sui blockchain wallet assistant. ONLY use the available tools when the user specifically and explicitly asks for wallet actions (balance, address, send, withdraw). DO NOT call any tools for greetings, casual conversation, or general questions. When you do use a tool, select the EXACT tool that matches the user's request - use get_wallet for address requests, get_balance for balance requests, send for transfer requests, and withdraw for withdrawal requests. Respond conversationally by default.";

        let request = Request::builder()
            .model(Model::GPT41Mini)  // Using GPT-4o Mini for efficient processing with tools
            .input(user_input)
            .instructions(instructions)
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

    /// Generates a response with conversation continuity and tools support
    /// This matches the exact pattern from the comprehensive demo
    pub async fn generate_with_tools_continuous(
        &self,
        user_input: &str,
        tools: Vec<Tool>,
        previous_response_id: Option<String>,
    ) -> Result<OAIResponse, CoreError> {
        let instructions = "You are SUI Squad Bot, a Sui blockchain wallet assistant. ONLY use the available tools when the user specifically and explicitly asks for wallet actions (balance, address, send, withdraw). DO NOT call any tools for greetings, casual conversation, or general questions. When you do use a tool, select the EXACT tool that matches the user's request - use get_wallet for address requests, get_balance for balance requests, send for transfer requests, and withdraw for withdrawal requests. Respond conversationally by default.";

        let mut request_builder = Request::builder()
            .model(Model::GPT41Mini)
            .input(user_input)
            .instructions(instructions)
            .tools(tools)
            .tool_choice(ToolChoice::auto());

        // Add conversation continuity if we have a previous conversation
        if let Some(prev_id) = previous_response_id {
            request_builder = request_builder.previous_response_id(prev_id);
        }

        let request = request_builder.build();

        let response = self
            .client
            .responses
            .create(request)
            .await
            .map_err(|e| CoreError::Other(format!("Failed to create response: {}", e)))?;

        println!("generate_with_tools_continuous result: {:?}", response);

        Ok(response)
    }

    /// Submit function outputs and continue the conversation
    /// Uses the exact pattern from the comprehensive demo
    pub async fn submit_tool_outputs(
        &self,
        response_id: String,
        function_outputs: Vec<(String, String)>, // (call_id, output)
        tools: Vec<Tool>,
    ) -> Result<OAIResponse, CoreError> {
        // Submit tool outputs and continue conversation using the exact pattern from demo
        let continuation_request = Request::builder()
            .model(Model::GPT41Mini)
            .with_function_outputs(response_id, function_outputs)
            .tools(tools) // Keep tools available for potential follow-ups
            .build();

        let response = self
            .client
            .responses
            .create(continuation_request)
            .await
            .map_err(|e| CoreError::Other(format!("Failed to create response: {}", e)))?;

        println!("submit_tool_outputs response: {:?}", response);

        Ok(response)
    }
}
