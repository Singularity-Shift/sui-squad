use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use teloxide::types::BotCommand;
use sui_squad_core::{
    config::Config,
    db::init_db,
    sui_gateway::DummyGateway,
    commands::bot_commands::Command,
};
use sui_squad_core::ai::openai_client::OpenAIClient;
use tracing_subscriber;
use anyhow::Result;

async fn answer(
    bot: Bot,
    msg: Message,
    cmd: Command,
) -> Result<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Login => bot.send_message(msg.chat.id, "Dummy response: You are now logged in!").await?,
        Command::GetWallet => bot.send_message(msg.chat.id, "Dummy response: Your wallet address is 0x123...").await?,
        Command::GetBalance(token) => {
            if token.trim().is_empty() {
                bot.send_message(msg.chat.id, "Dummy response: You have 100 SUI, 50 USDT.").await?
            } else {
                bot.send_message(msg.chat.id, format!("Dummy response: Your balance for {} is 100.", token)).await?
            }
        },
        Command::Prompt(prompt_text) => bot.send_message(msg.chat.id, format!("Dummy response: Processing prompt: {}", prompt_text)).await?,
        Command::PromptExamples => bot.send_message(msg.chat.id, "Dummy response: Examples:\n- /prompt What is the weather?\n- /prompt Summarize this article...").await?,
    };
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cfg = Config::from_env();
    let _pool = init_db(&cfg.database_url).await?;
    let _gateway = DummyGateway;
    let _ai_client = OpenAIClient::new(cfg.openai_api_key.clone());
    let bot = Bot::new(cfg.teloxide_token.clone());

    // Set command menu for Telegram
    let commands = vec![
        BotCommand::new("login", "Login to the service."),
        BotCommand::new("getwallet", "Get your wallet address."),
        BotCommand::new("getbalance", "Get your balance for all tokens or a specific token (e.g. /getbalance or /getbalance SUI)"),
        BotCommand::new("prompt", "Send a prompt to the AI."),
        BotCommand::new("promptexamples", "Show prompt examples."),
        BotCommand::new("help", "Display this help message."),
    ];
    bot.set_my_commands(commands).await?;

    let handler = Update::filter_message()
        .filter_command::<Command>()
        .endpoint(answer);

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}
