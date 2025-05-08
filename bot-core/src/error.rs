use thiserror::Error;

#[derive(Error, Debug)]
pub enum BotError {
    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),

    #[error("Unauthorized action")]
    Unauthorized,

    #[error("Gateway error: {0}")]
    GatewayError(String),

    #[error("Other error: {0}")]
    Other(String),
} 