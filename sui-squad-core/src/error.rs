use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Langchain operation failed: {0}")]
    LangchainError(String),

    #[error("Sui Client Initialization Error: {0}")]
    SuiClientInitializationError(String),

    #[error("Sui RPC Error: {0}")]
    SuiRpcError(String),

    #[error("Transaction Build Error: {0}")]
    TransactionBuildError(String),

    #[error("Object not found: {0}")]
    ObjectNotFound(String),

    #[error("Unauthorized action")]
    Unauthorized,

    #[error("Gateway error: {0}")]
    GatewayError(String),

    #[error("Other error: {0}")]
    Other(String),
} 