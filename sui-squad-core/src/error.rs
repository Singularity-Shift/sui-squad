use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Unauthorized action")]
    Unauthorized,

    #[error("OpenAI error: {0}")]
    OpenAiError(String),

    #[error("Other OpenAI error: {0}")]
    OpenAiOtherError(String),
} 