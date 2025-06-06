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
            parameters: None,
            function: None,
            vector_store_ids: Some(vec![]),
            container: None,
            server_label: None,
            server_url: None,
            headers: None,
            partial_images: None,
            require_approval: None,
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
                    "amount": { "type": "number", "description": "amount of SUI to withdraw (e.g., 1.5 for 1.5 SUI)" },
                    "address": { "type": "string", "description": "address to withdraw to" },
                },
                "required": ["amount", "address"],
                "additionalProperties": false
            })),
            function: None,
            vector_store_ids: Some(vec![]),
            container: None,
            server_label: None,
            server_url: None,
            headers: None,
            partial_images: None,
            require_approval: None,
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
                    "targets": { "type": "array","description": "telegram usernames without @ for example ['mytestuser', 'mytestuser2']", "items": { "type": "string" }},
                    "amount": { "type": "number","description": "amount of SUI to send (e.g., 1.5 for 1.5 SUI)" },
                },
                "required": ["targets", "amount"],
                "additionalProperties": false
            })),
            function: None,
            vector_store_ids: Some(vec![]),
            container: None,
            server_label: None,
            server_url: None,
            headers: None,
            partial_images: None,
            require_approval: None,
        },
    ]
}
