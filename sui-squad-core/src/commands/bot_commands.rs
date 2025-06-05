use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Command {
    Prompt(String),
    P(String),
    PromptExamples,
    Help,
    Fund,
}

impl Command {
    pub fn parse(text: &str) -> Option<Self> {
        let parts: Vec<&str> = text.splitn(2, ' ').collect();
        let command = parts[0];
        let args = parts.get(1).unwrap_or(&"").to_string();
        
        match command {
            "/prompt" => Some(Command::Prompt(args)),
            "/p" => Some(Command::P(args)),
            "/promptexamples" => Some(Command::PromptExamples),
            "/help" => Some(Command::Help),
            "/fund" => Some(Command::Fund),
            _ => None,
        }
    }
    
    pub fn description(&self) -> &'static str {
        match self {
            Command::Prompt(_) => "Send a prompt to the AI assistant.",
            Command::P(_) => "Send a prompt to the AI assistant (short alias for /prompt).",
            Command::PromptExamples => "Show Squard prompt examples.",
            Command::Help => "Display this help message.",
            Command::Fund => "Fund your account.",
        }
    }
}

pub type UserId = i64; // Telegram user ID

#[derive(Debug, Clone, Default)]
pub enum LoginState {
    #[default]
    Login,
    LocalStorate(HashMap<UserId, String>),
}
