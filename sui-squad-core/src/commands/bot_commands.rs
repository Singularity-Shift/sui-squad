use fastcrypto_zkp::bn254::zk_login::ZkLoginInputs;
use teloxide::macros::BotCommands;

#[derive(BotCommands, Clone, Debug)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "Send a prompt to the AI assistant.")]
    Prompt(String),
    #[command(description = "Send a prompt to the AI assistant (short alias for /prompt).")]
    P(String),
    #[command(description = "Show Squard prompt examples.")]
    PromptExamples,
    #[command(description = "Display this help message.")]
    Help,
}

#[derive(Debug, Clone, Default)]
pub enum LoginState {
    #[default]
    Login,
    Authenticated(ZkLoginInputs),
}
