use open_ai_rust_responses_by_sshift::types::Tool;
use serde_json::json;

pub fn get_schema() -> Vec<Tool> {
    vec![
        Tool {
            tool_type: "function".to_string(),
            name: Some("get_balance".to_string()),
            description: Some(
                "Get the user's balance for all tokens or a specific token".to_string(),
            ),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                },
                "required": [],
                "additionalProperties": false
            })),
            function: None,
            vector_store_ids: Some(vec![]),
            container: None,
            server_label: None,
            server_url: None,
            headers: None,
        },
        Tool {
            tool_type: "function".to_string(),
            name: Some("withdraw".to_string()),
            description: Some(
                "Withdraw a specified amount of a coin from the user's account".to_string(),
            ),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "amount": { "type": "string" },
                    "coin": { "type": "string" }
                },
                "required": ["amount", "coin"],
                "additionalProperties": false
            })),
            function: None,
            vector_store_ids: Some(vec![]),
            container: None,
            server_label: None,
            server_url: None,
            headers: None,
        },
        Tool {
            tool_type: "function".to_string(),
            name: Some("send".to_string()),
            description: Some(
                "Send a specified amount of a coin to a Telegram ID or everyone in the group"
                    .to_string(),
            ),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "target": { "type": "string", "description": "Telegram ID or 'everyone'" },
                    "amount": { "type": "string" },
                    "coin": { "type": "string" }
                },
                "required": ["target", "amount", "coin"],
                "additionalProperties": false
            })),
            function: None,
            vector_store_ids: Some(vec![]),
            container: None,
            server_label: None,
            server_url: None,
            headers: None,
        },
    ]
}
