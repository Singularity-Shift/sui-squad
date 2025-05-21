use openai_responses::types::Tool;
use serde_json::json;

pub fn get_schema() -> Vec<Tool> {
    vec![
        Tool::Function {
            name: "withdraw".to_string(),
            description: Some(
                "Withdraw a specified amount of a coin from the user's account".to_string(),
            ),
            parameters: json!({
                "type": "object",
                "properties": {
                    "amount": { "type": "string" },
                    "coin": { "type": "string" }
                },
                "required": ["amount", "coin"],
                "additionalProperties": false
            }),
            strict: true,
        },
        Tool::Function {
            name: "send".to_string(),
            description: Some(
                "Send a specified amount of a coin to a Telegram ID or everyone in the group"
                    .to_string(),
            ),
            parameters: json!({
                "type": "object",
                "properties": {
                    "target": { "type": "string", "description": "Telegram ID or 'everyone'" },
                    "amount": { "type": "string" },
                    "coin": { "type": "string" }
                },
                "required": ["target", "amount", "coin"],
                "additionalProperties": false
            }),
            strict: true,
        },
    ]
}
