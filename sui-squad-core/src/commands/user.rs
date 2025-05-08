use crate::sui_gateway::SuiGateway;

/// Handle user-specific commands like balance, pay, claim.
pub async fn handle_user_command<G: SuiGateway>(
    gateway: G,
    // TODO: add parameters for command context
) {
    // TODO: implement user command logic, e.g., balance_of, transfer, claim
} 