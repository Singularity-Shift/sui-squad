use crate::tools::{
    schema::get_schema,
    tools::{send_json, withdraw_json},
};
use anyhow::Result as AnyhowResult;
use open_ai_rust_responses_by_sshift::types::ResponseItem;
use reqwest::Url;
use squard_connect::client::squard_connect::SquardConnect;
use std::env;
use sui_squad_core::{ai::ResponsesClient, commands::bot_commands::LoginState, error::CoreError};
use teloxide::{
    dispatching::dialogue::InMemStorage, prelude::*, types::{InlineKeyboardButton, InlineKeyboardMarkup, Message, ParseMode}, Bot
};

use super::dto::State;

pub async fn handle_login(
    bot: Bot,
    msg: Message,
    squard_connect_client: SquardConnect,
) -> AnyhowResult<()> {
    let current_chat = msg.chat.clone();

    if !current_chat.is_group() {
        let user_id = msg.from().unwrap().id.to_string();

        let bot_id = bot.get_me().await.unwrap().id.to_string();
        println!("bot_id: {}", bot_id);

        let mut squard_connect_client = squard_connect_client.clone();

        squard_connect_client.create_zkp_payload().await?;

        let (network, public_key, max_epoch, randomness) = squard_connect_client.get_zk_proof_params();

        let state = State::from((user_id.to_string(), bot_id, network.to_string(), public_key, max_epoch, randomness));

        let host = env::var("HOST").expect("HOST env variable is not set");
        let redirect_url = format!("https://{host}/webhook/token");
        

        let url_to_build = squard_connect_client.get_url::<State>(redirect_url, Some(state)).await?;

        let url = Url::parse(&url_to_build).unwrap();

        let login_button = vec![vec![InlineKeyboardButton::new(
            "Login with google",
            teloxide::types::InlineKeyboardButtonKind::Url(url),
        )]];

        let markdown = InlineKeyboardMarkup::new(login_button);

        bot.send_message(current_chat.id, "Login on Google account")
            .reply_markup(markdown)
            .await?;
    }

    Ok(())
}

pub async fn handle_prompt(
    bot: Bot,
    msg: Message,
    prompt_text: String,
    responses_client: ResponsesClient,
    dialogue: Dialogue<LoginState, InMemStorage<LoginState>>,
    squard_connect_client: SquardConnect,
) -> AnyhowResult<Message> {
    // Define custom function/tool schemas for the model
    let schema = get_schema();
    // Call AI with function-calling enabled
    match responses_client
        .generate_with_tools(&prompt_text, schema)
        .await
    {
        Ok(response) => {
            // Check if the model requested a function call
            // Try different possible ResponseItem variants to find tool calls
            if let Some(function_call) = response.output.iter().find_map(|item| {
                match item {
                    ResponseItem::FunctionCall { name, arguments, call_id, .. } => {
                        Some((name, arguments, call_id))
                    },
                    _ => {
                        println!("Response item: {:?}", item);
                    None
                    },
                }
            }) {
                // Extract function call information
                let (function_name, args_str, _call_id) = function_call;
                
                println!("Function call detected: {} with args: {:?}", function_name, args_str);
                
                // Parse the JSON arguments
                let args: serde_json::Value = serde_json::from_str(args_str)
                    .unwrap_or_else(|_| serde_json::json!({}));
                
                // Dispatch to our handlers
                let message = match function_name.as_str() {
                    "get_wallet" => {
                        handle_get_wallet_tool(dialogue.clone(), squard_connect_client.clone()).await
                    }
                    "get_balance" => {
                        let token = args.get("token").and_then(|v| v.as_str()).map(String::from);
                        handle_get_balance_tool(dialogue.clone(), squard_connect_client.clone(), token).await
                    }
                    "withdraw" => {
                        withdraw_json(&args)
                    }
                    "send" => {
                        send_json(&args)
                    }
                    _ => format!("Unknown function call: {}", function_name),
                };
                
                // Ensure we always have a non-empty message
                let final_message = if message.trim().is_empty() {
                    format!("Executed {} function but got empty response", function_name)
                } else {
                    message
                };
                
                let message = bot.send_message(msg.chat.id, final_message)
                    .parse_mode(ParseMode::MarkdownV2)
                    .await?;

                Ok(message)
            } else {
                // No function call: send direct model output
                let message = bot
                    .send_message(msg.chat.id, response.output_text())
                    .parse_mode(ParseMode::MarkdownV2)
                    .await?;

                Ok(message)
            }
        }
        Err(e) => {
            println!("Error processing prompt: {:?}", e);
            let user_message = match e {
                CoreError::ConfigurationError(s) => format!("AI configuration error: {}", s),
                CoreError::LangchainError(s) => format!("AI processing error: {}", s),
                CoreError::Other(s) => format!("AI processing error: {}", s),
                _ => "Sorry, I couldn't process your prompt due to an internal error.".to_string(),
            };
            let message = bot.send_message(msg.chat.id, user_message).await?;

            Ok(message)
        }
    }
}

pub async fn handle_get_wallet_tool(
    dialogue: Dialogue<LoginState, InMemStorage<LoginState>>,
    squard_connect_client: SquardConnect,
) -> String {
    let state = dialogue.get().await.unwrap();

    if let Some(LoginState::Authenticated(zk_login_inputs)) = state {
        let sender = squard_connect_client.get_sender(zk_login_inputs);
        format!("Your wallet address is:\n`{}`", sender.to_string())
    } else {
        "Error getting wallet address. Please login first.".to_string()
    }
}

pub async fn handle_get_balance_tool(
    dialogue: Dialogue<LoginState, InMemStorage<LoginState>>,
    squard_connect_client: SquardConnect,
    token: Option<String>,
) -> String {
    let mut balance_opt = None;
    let state = dialogue.get().await.unwrap();

    if let Some(LoginState::Authenticated(zk_login_inputs)) = state {
        let sender = squard_connect_client.get_sender(zk_login_inputs);
        let node = squard_connect_client.get_node();

        let balance_result = node.coin_read_api().get_balance(sender, None).await;
        if let Ok(balance) = balance_result {
            balance_opt = Some(balance);
        } else {
            println!("Error getting balance: {:?}", balance_result.err());
        }
    }

    match balance_opt {
        Some(balance) => {
            if let Some(token_name) = token {
                format!("Your {} balance is {}", token_name, balance.total_balance)
            } else {
                format!("Your total balance is {}", balance.total_balance)
            }
        }
        None => "Error getting balance. Please login first.".to_string(),
    }
}