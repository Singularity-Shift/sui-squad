use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use teloxide::types::BotCommand;
use sui_squad_core::{
    config::Config,
    db::init_db,
    sui_gateway::DummyGateway,
    commands::bot_commands::Command,
    ai::OpenAiClient,
    error::CoreError,
};
use tracing_subscriber;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;

async fn answer(
    bot: Bot,
    msg: Message,
    cmd: Command,
    ai_client: Arc<Mutex<OpenAiClient>>,
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
        Command::Prompt(prompt_text) => {
            let mut ai_client = ai_client.lock().await;
            match ai_client.generate_response(&prompt_text).await {
                Ok(response) => {
                    bot.send_message(msg.chat.id, response).await?
                }
                Err(e) => {
                    eprintln!("Error processing prompt: {:?}", e);
                    let user_message = match e {
                        CoreError::ConfigurationError(s) => format!("AI configuration error: {}", s),
                        _ => "Sorry, I couldn't process your prompt due to an internal error.".to_string(),
                    };
                    bot.send_message(msg.chat.id, user_message).await?
                }
            }
        },
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
    let ai_client = Arc::new(Mutex::new(OpenAiClient::new(&cfg)?));
    let bot = Bot::new(cfg.teloxide_token.clone());

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
        .dependencies(dptree::deps![ai_client])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}
