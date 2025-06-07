use std::collections::HashMap;

use teloxide::{macros::BotCommands, types::UserId};

#[derive(BotCommands, Clone, Debug)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "Log in to your account.")]
    Login,
    #[command(description = "Send a prompt to the AI assistant.")]
    Prompt(String),
    #[command(description = "Send a prompt to the AI assistant (short alias for /prompt).")]
    P(String),
    #[command(description = "Show Squard prompt examples.")]
    PromptExamples,
    #[command(description = "Display this help message.")]
    Help,
    #[command(description = "Fund your account.")]
    Fund,
}

#[derive(Debug, Clone, Default)]
pub enum LoginState {
    #[default]
    Login,
    LocalStorate(HashMap<UserId, String>),
}
