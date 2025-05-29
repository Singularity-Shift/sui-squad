pub mod responses_client;

pub use responses_client::ResponsesClient;

use open_ai_rust_responses_by_sshift::types::{Response as OAIResponse, ResponseItem};

// Extension trait to extract tool calls from response
pub trait ResponseExt {
    fn tool_calls(&self) -> Vec<ToolCall>;
}

#[derive(Debug, Clone)]
pub struct ToolCall {
    pub name: String,
    pub arguments: String,
    pub call_id: String,
}

impl ResponseExt for OAIResponse {
    fn tool_calls(&self) -> Vec<ToolCall> {
        self.output.iter().filter_map(|item| {
            match item {
                ResponseItem::FunctionCall { name, arguments, call_id, .. } => {
                    Some(ToolCall {
                        name: name.clone(),
                        arguments: arguments.clone(),
                        call_id: call_id.clone(),
                    })
                },
                _ => None,
            }
        }).collect()
    }
} 