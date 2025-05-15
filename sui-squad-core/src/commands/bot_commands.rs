use teloxide::macros::BotCommands;

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "Display this help message.")]
    Help,
    #[command(description = "Login to the service.")]
    Login,
    #[command(description = "Get your wallet address.")]
    GetWallet,
    #[command(description = "Get your balance for all tokens or a specific token (e.g. /get_balance or /get_balance SUI)")]
    GetBalance(String),
    #[command(description = "Send a prompt to the AI.")]
    Prompt(String),
    #[command(description = "Show prompt examples.")]
    PromptExamples,
} 