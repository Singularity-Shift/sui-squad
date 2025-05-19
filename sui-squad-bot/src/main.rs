use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use teloxide::types::BotCommand;
use sui_squad_core::{
    config::Config,
    db::init_db,
    sui_gateway::DummyGateway,
    commands::bot_commands::Command,
    ai::ResponsesClient,
    error::CoreError,
};
use tracing_subscriber;
use anyhow::Result;

mod handlers;
use openai_responses::types::{Tool, OutputItem};
use serde_json::json;

async fn answer(
    bot: Bot,
    msg: Message,
    cmd: Command,
    responses_client: ResponsesClient,
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
            // Define custom function/tool schemas for the model
            let tools = vec![
                Tool::Function {
                    name: "withdraw".to_string(),
                    description: Some("Withdraw a specified amount of a coin from the user's account".to_string()),
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
                    description: Some("Send a specified amount of a coin to a Telegram ID or everyone in the group".to_string()),
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
            ];
            // Call AI with function-calling enabled
            match responses_client.generate_with_tools(&prompt_text, tools).await {
                Ok(response) => {
                    // Check if the model requested a function call
                    if let Some(call) = response.output.iter().find_map(|item| {
                        if let OutputItem::FunctionCall(call) = item {
                            Some(call)
                        } else {
                            None
                        }
                    }) {
                        // Parse the arguments JSON
                        let args: serde_json::Value = serde_json::from_str(&call.arguments).unwrap_or_default();
                        // Dispatch to our handlers
                        let message = match call.name.as_str() {
                            "withdraw" => {
                                let amount = args.get("amount").and_then(|v| v.as_str()).unwrap_or("");
                                let coin = args.get("coin").and_then(|v| v.as_str()).unwrap_or("");
                                handlers::handle_withdraw(&format!("withdraw {} {}", amount, coin))
                            }
                            "send" => {
                                let target = args.get("target").and_then(|v| v.as_str()).unwrap_or("");
                                let amount = args.get("amount").and_then(|v| v.as_str()).unwrap_or("");
                                let coin = args.get("coin").and_then(|v| v.as_str()).unwrap_or("");
                                handlers::handle_send(&format!("send {} {} {}", target, amount, coin))
                            }
                            _ => "Unknown function call".to_string(),
                        };
                        bot.send_message(msg.chat.id, message).await?
                    } else {
                        // No function call: send direct model output
                        bot.send_message(msg.chat.id, response.output_text()).await?
                    }
                }
                Err(e) => {
                    eprintln!("Error processing prompt: {:?}", e);
                    let user_message = match e {
                        CoreError::ConfigurationError(s) => format!("AI configuration error: {}", s),
                        CoreError::LangchainError(s) => format!("AI processing error: {}", s),
                        CoreError::Other(s) => format!("AI processing error: {}", s),
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
    let responses_client = ResponsesClient::new(&cfg)?;
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
        .dependencies(dptree::deps![responses_client.clone()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}
