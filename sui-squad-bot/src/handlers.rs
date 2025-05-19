/// Parses a "withdraw" command and returns a dummy response.
/// Expected format: "withdraw <amount> <coin>"
pub fn handle_withdraw(input: &str) -> String {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() == 3 {
        let amount = parts[1];
        let coin = parts[2];
        format!("Withdrew {} {}", amount, coin)
    } else {
        "Invalid withdraw command. Usage: withdraw <amount> <coin>".to_string()
    }
}

/// Parses a "send" command and returns a dummy response.
/// Handles both specific user and group sends.
/// Formats:
/// - "send <telegram_id> <amount> <coin>"
/// - "send everyone in the group <amount> <coin>"
pub fn handle_send(input: &str) -> String {
    let lower = input.trim().to_lowercase();
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if lower.starts_with("send") {
        if lower.contains("everyone") {
            if parts.len() >= 2 {
                // amount is the second-to-last, coin is last
                let amount = parts[parts.len() - 2];
                let coin = parts[parts.len() - 1];
                return format!("Sent {} {} to everyone in the group", amount, coin);
            }
        } else if parts.len() == 4 {
            let target = parts[1];
            let amount = parts[2];
            let coin = parts[3];
            return format!("Sent {} {} to {}", amount, coin, target);
        }
    }
    "Invalid send command. Usage: send <telegram_id> <amount> <coin> or send everyone in the group <amount> <coin>".to_string()
} 