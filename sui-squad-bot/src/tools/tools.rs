use serde_json::Value;

pub fn withdraw_json(args: &Value) -> String {
    let amount = args.get("amount").and_then(|v| v.as_str()).unwrap_or("");
    let coin = args.get("coin").and_then(|v| v.as_str()).unwrap_or("");
    
    if !amount.is_empty() && !coin.is_empty() {
        format!("Withdrew {} {}", amount, coin)
    } else {
        "Invalid withdraw command. Missing amount or coin.".to_string()
    }
}

pub fn send_json(args: &Value) -> String {
    let target = args.get("target").and_then(|v| v.as_str()).unwrap_or("");
    let amount = args.get("amount").and_then(|v| v.as_str()).unwrap_or("");
    let coin = args.get("coin").and_then(|v| v.as_str()).unwrap_or("");
    
    if !target.is_empty() && !amount.is_empty() && !coin.is_empty() {
        if target.to_lowercase() == "everyone" {
            format!("Sent {} {} to everyone in the group", amount, coin)
        } else {
            format!("Sent {} {} to {}", amount, coin, target)
        }
    } else {
        "Invalid send command. Missing target, amount, or coin.".to_string()
    }
}
