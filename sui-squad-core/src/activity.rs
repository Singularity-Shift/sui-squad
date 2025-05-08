use sqlx::SqlitePool;
use crate::error::BotError;

/// Increment activity counter for a user in a group.
pub async fn increment_activity(
    pool: &SqlitePool,
    user_id: i64,
    group_id: i64,
) -> Result<(), BotError> {
    // TODO: implement increment logic, with weekly resets
    Ok(())
}

/// Get top N users by activity.
pub async fn top(
    pool: &SqlitePool,
    group_id: i64,
    n: usize,
) -> Result<Vec<(i64, u64)>, BotError> {
    // TODO: query top N users
    Ok(vec![])
} 