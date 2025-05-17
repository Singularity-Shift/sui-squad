use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use teloxide::types::BotCommand;
use sui_squad_core::{
    config::Config,
    db::init_db,
    sui_gateway::LiveSuiGateway,
    sui_gateway::SuiGateway,
    commands::bot_commands::Command,
    ai::LangchainClient,
    error::CoreError,
};
use tracing_subscriber;
use anyhow::Result;
use std::sync::Arc;
use sui_sdk::types::base_types::AccountAddress;
use sui_sdk::types::TypeTag;
use sui_sdk::types::struct_tag::StructTag;
use std::str::FromStr;
use rusqlite::SqlitePool;

const SUI_FRAMEWORK_ADDRESS_BYTES: [u8; AccountAddress::LENGTH] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2];
const SUI_MODULE_NAME: &str = "sui";
const SUI_TYPE_NAME: &str = "SUI";

fn get_sui_coin_type_tag() -> TypeTag {
    TypeTag::Struct(Box::new(StructTag {
        address: AccountAddress::new(SUI_FRAMEWORK_ADDRESS_BYTES),
        module: sui_sdk::types::Identifier::new(SUI_MODULE_NAME).unwrap(),
        name: sui_sdk::types::Identifier::new(SUI_TYPE_NAME).unwrap(),
        type_params: vec![],
    }))
}

async fn answer(
    bot: Bot,
    msg: Message,
    cmd: Command,
    ai_client: LangchainClient,
    gateway: Arc<LiveSuiGateway>,
    db_pool: Arc<SqlitePool>,
) -> Result<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Login => bot.send_message(msg.chat.id, "Dummy response: You are now logged in!").await?,
        Command::GetWallet => {
            let user_id_str = msg.from().map_or_else(|| "unknown_user".to_string(), |u| u.id.0.to_string());
            let chat_id_str = msg.chat.id.to_string();

            match gateway.get_sui_address_for_telegram_user(user_id_str, chat_id_str).await {
                Ok(Some(addr)) => bot.send_message(msg.chat.id, format!("Your SUI address is: {}", addr)).await?,
                Ok(None) => bot.send_message(msg.chat.id, "Your SUI address is not yet linked. Use /linkaddress <your_sui_address>.").await?,
                Err(e) => {
                    eprintln!("Error getting SUI address: {:?}", e);
                    bot.send_message(msg.chat.id, "Could not retrieve your SUI address due to an error.").await?
                }
            }
        },
        Command::GetBalance(token_symbol) => {
            let user_id_str = msg.from().map_or_else(|| "unknown_user".to_string(), |u| u.id.0.to_string());
            let chat_id_str = msg.chat.id.to_string();

            if !token_symbol.trim().is_empty() && token_symbol.to_uppercase() != "SUI" {
                bot.send_message(msg.chat.id, format!("Sorry, balance check for token '{}' is not yet supported. Try /getbalance SUI or /getbalance.", token_symbol)).await?;
                return Ok(());
            }
            let coin_type_tag = get_sui_coin_type_tag();

            match gateway.get_account_object_id_for_telegram_user(user_id_str.clone(), chat_id_str.clone()).await {
                Ok(Some(account_object_id_str)) => {
                    let account_object_id = account_object_id_str;

                    match gateway.get_account_balance(account_object_id, coin_type_tag).await {
                        Ok(balance) => {
                            let balance_sui = balance as f64 / 1_000_000_000.0;
                            bot.send_message(msg.chat.id, format!("Your balance for SUI is: {:.9} SUI", balance_sui)).await?
                        },
                        Err(e) => {
                            eprintln!("Error getting balance: {:?}", e);
                            bot.send_message(msg.chat.id, "Could not retrieve your balance due to an error.").await?
                        }
                    }
                },
                Ok(None) => {
                    bot.send_message(msg.chat.id, "Your account is not yet set up on-chain. Please use /createaccount first.").await?
                },
                Err(e) => {
                    eprintln!("Error getting account object ID: {:?}", e);
                    bot.send_message(msg.chat.id, "Could not retrieve your account details due to an error.").await?
                }
            }
        },
        Command::Prompt(prompt_text) => {
            match ai_client.generate_response(&prompt_text).await {
                Ok(response) => {
                    bot.send_message(msg.chat.id, response).await?
                }
                Err(e) => {
                    eprintln!("Error processing prompt: {:?}", e);
                    let user_message = match e {
                        CoreError::ConfigurationError(s) => format!("AI configuration error: {}", s),
                        CoreError::LangchainError(s) => format!("AI processing error: {}", s),
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
    let db_pool = Arc::new(init_db(&cfg.database_url).await?);
    let gateway = Arc::new(LiveSuiGateway::new(cfg.clone(), db_pool.clone()).await?);
    let ai_client = LangchainClient::new(&cfg)?;
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
        .dependencies(dptree::deps![ai_client.clone(), gateway.clone(), db_pool.clone()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}
