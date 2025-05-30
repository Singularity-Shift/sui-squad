use crate::config::Config;
use crate::error::CoreError;
use open_ai_rust_responses_by_sshift::{Client as OAIClient, Request, Model};
use open_ai_rust_responses_by_sshift::types::{
    Tool, Response as OAIResponse, ToolChoice,
};

/// Unified system prompt for all interactions
const SYSTEM_PROMPT: &str = "You are SUI Squad Bot, a Sui blockchain wallet assistant for Telegram groups! 🚀 Be enthusiastic, friendly, and engaging with light use of emojis. Make genuine connections with users and mirror their communication style and energy level. Respond conversationally and provide helpful wallet information.

When tools are available, ONLY use them when users specifically ask for wallet actions (balance, address, send, withdraw). DO NOT call tools for greetings or casual conversation. Match tools exactly: get_wallet for addresses, get_balance for balances, send for transfers, withdraw for withdrawals.

CRITICAL: When processing function results, you MUST preserve HTML formatting EXACTLY as provided. Do NOT convert <code></code> tags to backticks or any other format. Do NOT escape or modify HTML tags. Output function results with their HTML intact and add your own enthusiastic response with emojis around them.";

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

    /// Unified method to generate responses with support for:
    /// - Basic text responses (user_input only)
    /// - Tool/function calling (with tools)
    /// - Conversation continuity (with previous_response_id)
    /// - Function output submission (with response_id and function_outputs)
    /// 
    /// # Usage Examples:
    /// 
    /// Basic text response:
    /// ```ignore
    /// let response = client.generate_response(
    ///     Some("Hello"), 
    ///     None, 
    ///     None, 
    ///     None
    /// ).await?;
    /// ```
    /// 
    /// With tools (function calling):
    /// ```ignore
    /// let response = client.generate_response(
    ///     Some("What's my balance?"), 
    ///     Some(tools), 
    ///     None, 
    ///     None
    /// ).await?;
    /// ```
    /// 
    /// With conversation continuity:
    /// ```ignore
    /// let response = client.generate_response(
    ///     Some("Continue our chat"), 
    ///     Some(tools), 
    ///     Some("prev_response_id".to_string()), 
    ///     None
    /// ).await?;
    /// ```
    /// 
    /// Submit function outputs:
    /// ```ignore
    /// let response = client.generate_response(
    ///     None, 
    ///     Some(tools), 
    ///     None, 
    ///     Some(("response_id".to_string(), vec![("call_id".to_string(), "output".to_string())]))
    /// ).await?;
    /// ```
    pub async fn generate_response(
        &self,
        user_input: Option<&str>,
        tools: Option<Vec<Tool>>,
        previous_response_id: Option<String>,
        function_outputs: Option<(String, Vec<(String, String)>)>, // (response_id, outputs)
    ) -> Result<OAIResponse, CoreError> {
        
        // Handle function output submission case
        if let Some((response_id, outputs)) = function_outputs {
            return self.submit_function_outputs(response_id, outputs, tools.unwrap_or_default()).await;
        }

        // Regular response generation (with or without tools and continuity)
        let user_input = user_input.ok_or_else(|| 
            CoreError::Other("user_input is required when not submitting function outputs".to_string())
        )?;

        let mut request_builder = Request::builder()
            .model(Model::GPT41Mini)
            .input(user_input)
            .instructions(SYSTEM_PROMPT);

        // Add tools if provided
        if let Some(tools_vec) = tools {
            request_builder = request_builder
                .tools(tools_vec)
                .tool_choice(ToolChoice::auto());
        }

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

        println!("generate_response result: {:?}", response);

        Ok(response)
    }

    /// Internal helper method for submitting function outputs
    async fn submit_function_outputs(
        &self,
        response_id: String,
        function_outputs: Vec<(String, String)>, // (call_id, output)
        tools: Vec<Tool>,
    ) -> Result<OAIResponse, CoreError> {
        // Submit tool outputs and continue conversation using the exact pattern from demo
        let continuation_request = Request::builder()
            .model(Model::GPT41Mini)
            .with_function_outputs(response_id, function_outputs)
            .instructions(SYSTEM_PROMPT) // CRITICAL: Include system instructions for function output processing
            .tools(tools) // Keep tools available for potential follow-ups
            .build();

        let response = self
            .client
            .responses
            .create(continuation_request)
            .await
            .map_err(|e| CoreError::Other(format!("Failed to create response: {}", e)))?;

        println!("submit_function_outputs response: {:?}", response);

        Ok(response)
    }

    // DEPRECATED: These methods remain for backward compatibility but delegate to generate_response()
    
    /// DEPRECATED: Use generate_response() instead
    #[deprecated(note = "Use generate_response() with appropriate parameters instead")]
    pub async fn generate_with_tools(
        &self,
        user_input: &str,
        tools: Vec<Tool>,
    ) -> Result<OAIResponse, CoreError> {
        self.generate_response(Some(user_input), Some(tools), None, None).await
    }

    /// DEPRECATED: Use generate_response() instead
    #[deprecated(note = "Use generate_response() with appropriate parameters instead")]
    pub async fn generate_with_tools_continuous(
        &self,
        user_input: &str,
        tools: Vec<Tool>,
        previous_response_id: Option<String>,
    ) -> Result<OAIResponse, CoreError> {
        self.generate_response(Some(user_input), Some(tools), previous_response_id, None).await
    }

    /// DEPRECATED: Use generate_response() instead
    #[deprecated(note = "Use generate_response() with function_outputs parameter instead")]
    pub async fn submit_tool_outputs(
        &self,
        response_id: String,
        function_outputs: Vec<(String, String)>, // (call_id, output)
        tools: Vec<Tool>,
    ) -> Result<OAIResponse, CoreError> {
        self.generate_response(None, Some(tools), None, Some((response_id, function_outputs))).await
    }

    // CONVENIENCE METHODS - These make common use cases easier to call
    
    /// Simple text-only response (no tools, no continuity)
    pub async fn simple_response(&self, user_input: &str) -> Result<OAIResponse, CoreError> {
        self.generate_response(Some(user_input), None, None, None).await
    }
    
    /// Response with tools enabled (for function calling)
    pub async fn with_tools(&self, user_input: &str, tools: Vec<Tool>) -> Result<OAIResponse, CoreError> {
        self.generate_response(Some(user_input), Some(tools), None, None).await
    }
    
    /// Continue conversation with tools
    pub async fn continue_conversation(
        &self, 
        user_input: &str, 
        tools: Vec<Tool>, 
        previous_response_id: String
    ) -> Result<OAIResponse, CoreError> {
        self.generate_response(Some(user_input), Some(tools), Some(previous_response_id), None).await
    }
    
    /// Submit function outputs and continue
    pub async fn submit_outputs(
        &self, 
        response_id: String, 
        outputs: Vec<(String, String)>, 
        tools: Vec<Tool>
    ) -> Result<OAIResponse, CoreError> {
        self.generate_response(None, Some(tools), None, Some((response_id, outputs))).await
    }
}
