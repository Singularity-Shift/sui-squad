use sqlx::SqlitePool;
use crate::error::CoreError;

/// Increment activity counter for a user in a group.
pub async fn increment_activity(
    _pool: &SqlitePool,
    _user_id: i64,
    _group_id: i64,
) -> Result<(), CoreError> {
    // TODO: implement increment logic, with weekly resets
    Ok(())
}

/// Get top N users by activity.
pub async fn top(
    _pool: &SqlitePool,
    _group_id: i64,
    _n: usize,
) -> Result<Vec<(i64, u64)>, CoreError> {
    // TODO: query top N users
    Ok(vec![])
} 