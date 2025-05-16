use langchain_rust::{
    chain::{Chain, LLMChainBuilder},
    fmt_message, fmt_template,
    llm::openai::{OpenAI, OpenAIModel, OpenAIConfig},
    message_formatter,
    prompt::HumanMessagePromptTemplate,
    prompt_args,
    schemas::messages::Message,
    template_fstring,
};
use crate::config::Config; // Assuming your config loader is here
use crate::error::CoreError; // Assuming a custom error type

/// Enum representing parsed intents from natural language inputs.
#[derive(Debug)]
pub enum Intent {
    GetBalance,
    GetWallet,
    Prompt,
    // Add other intents as needed
}

/// Client for OpenAI-based intent parsing.
#[derive(Clone)]
pub struct LangchainClient {
    llm: OpenAI<OpenAIConfig>,
    // We might not need a separate reqwest client if langchain handles it.
}

impl LangchainClient {
    pub fn new(config: &Config) -> Result<Self, CoreError> {
        let api_key = config.openai_api_key().ok_or_else(|| CoreError::ConfigurationError("OpenAI API key not found".to_string()))?;
        let open_ai_config = OpenAIConfig::default().with_api_key(api_key);
        let llm = OpenAI::default()
            .with_config(open_ai_config)
            .with_model(OpenAIModel::Gpt4o.to_string());

        Ok(LangchainClient { llm })
    }

    pub async fn generate_response(&self, user_input: &str) -> Result<String, CoreError> {
        // A simple prompt - this can be made much more sophisticated
        let prompt = message_formatter![
            fmt_message!(Message::new_system_message(
                "You are a helpful assistant for the Sui Squad Telegram bot."
            )),
            fmt_template!(HumanMessagePromptTemplate::new(template_fstring!(
                "{input}", "input"
            )))
        ];

        let chain = LLMChainBuilder::new()
            .prompt(prompt)
            .llm(self.llm.clone()) // LLM in langchain-rust is cloneable
            .build()
            .map_err(|e| CoreError::LangchainError(e.to_string()))?;

        let result = chain
            .invoke(prompt_args! { "input" => user_input })
            .await
            .map_err(|e| CoreError::LangchainError(e.to_string()))?;

        Ok(result)
    }

    // If you still want to parse specific intents, you'd adapt this.
    // This could involve a more complex prompt engineering step.
    pub async fn parse_intent(&self, _text: &str) -> Result<Option<Intent>, CoreError> {
        // For now, let's say all prompts go to the general response.
        // Intent parsing with LLMs usually involves asking the LLM to classify the input
        // or to return a structured JSON. Langchain has tools for this (e.g., output parsers).
        // This is a placeholder and would need significant expansion for robust intent parsing.
        println!("Intent parsing with Langchain would be implemented here.");
        Ok(None) // Placeholder
    }
}

// Example of how you might update your CoreError enum
// Ensure this is defined in your src/error.rs
/*
#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Langchain operation failed: {0}")]
    LangchainError(String),

    // other errors
}
*/ 