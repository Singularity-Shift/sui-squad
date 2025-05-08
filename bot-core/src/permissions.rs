/// Check if a user ID is in the list of admin IDs.
pub fn is_admin(user_id: i64, admins: &[i64]) -> bool {
    admins.contains(&user_id)
}

/// Macro to require admin privileges in a function context.
#[macro_export]
macro_rules! require_admin {
    ($user_id:expr, $admins:expr) => {
        if !crate::permissions::is_admin($user_id, $admins) {
            return Err(crate::error::BotError::Unauthorized);
        }
    };
} 