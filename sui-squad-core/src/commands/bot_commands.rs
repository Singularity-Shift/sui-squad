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
    #[command(description = "Admin: Create a new group on-chain. Args: <telegram_group_id_string>")]
    AdminCreateGroup(String),
    #[command(description = "Admin: Link a user SUI address to their Telegram ID for a group. Args: <user_sui_address> <telegram_user_id_string> <telegram_group_id_string>")]
    AdminLinkUser(String, String, String),
    #[command(description = "Create your on-chain SUI account managed by the bot.")]
    CreateAccount,
} 