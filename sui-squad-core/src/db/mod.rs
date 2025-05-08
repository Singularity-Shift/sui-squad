pub mod migrations;

use sqlx::SqlitePool;
use crate::error::BotError;

/// Initialize the SQLite database and run migrations.
pub async fn init_db(database_url: &str) -> Result<SqlitePool, BotError> {
    let pool = SqlitePool::connect(database_url).await?;
    // TODO: run migrations
    Ok(pool)
} 