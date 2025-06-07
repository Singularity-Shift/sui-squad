use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Langchain operation failed: {0}")]
    LangchainError(String),

    #[error("Unauthorized action")]
    Unauthorized,

    #[error("Gateway error: {0}")]
    GatewayError(String),

    #[error("Other error: {0}")]
    Other(String),
}
