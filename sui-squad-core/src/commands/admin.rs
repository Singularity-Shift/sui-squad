use crate::sui_gateway::SuiGateway;

/// Handle admin-specific commands, calling gateway methods as needed.
pub async fn handle_admin_command<G: SuiGateway>(
    _command: &str,
    _gateway: G,
    // TODO: add parameters for command context
) -> String {
    // TODO: implement admin command logic, e.g., create pool, reward top users
    String::new()
} 