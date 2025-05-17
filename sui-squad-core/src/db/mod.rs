pub mod migrations;

use sqlx::{SqlitePool, FromRow};
use crate::error::CoreError;
use serde::{Serialize, Deserialize};

// Struct for the user_sui_map table
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserSuiMap {
    pub telegram_user_id: i64,      // Telegram user ID (PK part 1)
    pub telegram_group_id: String,  // Telegram group ID (PK part 2)
    pub sui_address: String,        // User's SUI address
    pub sui_account_object_id: Option<String>, // ObjectID of the user's on-chain Account, if created
}

// Struct for the sui_groups_map table
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SuiGroupMap {
    pub telegram_group_id: String, // Telegram group ID (PK)
    pub sui_group_object_id: String, // ObjectID of the on-chain Group object
}

/// Initialize the SQLite database and run migrations.
pub async fn init_db(database_url: &str) -> Result<SqlitePool, CoreError> {
    let pool = SqlitePool::connect(database_url).await?;
    // TODO: run migrations
    sqlx::migrate!("./migrations") // Assuming migrations are in a top-level migrations folder relative to CARGO_MANIFEST_DIR
        .run(&pool)
        .await
        .map_err(|e| CoreError::DbError(sqlx::Error::Migrate(Box::new(e))))?;
    Ok(pool)
}

// --- UserSuiMap Functions ---

pub async fn store_user_sui_map(pool: &SqlitePool, mapping: &UserSuiMap) -> Result<(), CoreError> {
    sqlx::query_unchecked!(
        r#"
        INSERT OR REPLACE INTO user_sui_map (telegram_user_id, telegram_group_id, sui_address, sui_account_object_id)
        VALUES (?, ?, ?, ?)
        "#,
        mapping.telegram_user_id,
        mapping.telegram_group_id,
        mapping.sui_address,
        mapping.sui_account_object_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_user_sui_map(pool: &SqlitePool, telegram_user_id: i64, telegram_group_id: &str) -> Result<Option<UserSuiMap>, CoreError> {
    let mapping = sqlx::query_as_unchecked!(
        UserSuiMap,
        r#"
        SELECT telegram_user_id, telegram_group_id, sui_address, sui_account_object_id
        FROM user_sui_map
        WHERE telegram_user_id = ? AND telegram_group_id = ?
        "#,
        telegram_user_id,
        telegram_group_id
    )
    .fetch_optional(pool)
    .await?;
    Ok(mapping)
}

// --- SuiGroupMap Functions ---

pub async fn store_sui_group_map(pool: &SqlitePool, mapping: &SuiGroupMap) -> Result<(), CoreError> {
    sqlx::query_unchecked!(
        r#"
        INSERT OR REPLACE INTO sui_groups_map (telegram_group_id, sui_group_object_id)
        VALUES (?, ?)
        "#,
        mapping.telegram_group_id,
        mapping.sui_group_object_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_sui_group_map(pool: &SqlitePool, telegram_group_id: &str) -> Result<Option<SuiGroupMap>, CoreError> {
    let mapping = sqlx::query_as_unchecked!(
        SuiGroupMap,
        r#"
        SELECT telegram_group_id, sui_group_object_id
        FROM sui_groups_map
        WHERE telegram_group_id = ?
        "#,
        telegram_group_id
    )
    .fetch_optional(pool)
    .await?;
    Ok(mapping)
} 