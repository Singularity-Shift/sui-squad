use crate::tools::{
    schema::get_schema,
    tools::{send, withdraw},
};
use anyhow::Result as AnyhowResult;
use openai_responses::types::OutputItem;
use reqwest::Url;
use squard_connect::client::squard_connect::SquardConnect;
use std::env;
use sui_squad_core::{ai::ResponsesClient, commands::bot_commands::LoginState, error::CoreError};
use teloxide::{
    dispatching::dialogue::InMemStorage, prelude::*, types::{InlineKeyboardButton, InlineKeyboardMarkup, Message}, Bot, RequestError
};

use super::dto::State;

pub async fn handle_login(
    bot: Bot,
    msg: Message,
    dialogue: Dialogue<LoginState, InMemStorage<LoginState>>,
    squard_connect_client: SquardConnect,
) -> AnyhowResult<()> {
    let current_chat = msg.chat;

    if !current_chat.is_group() {
        let username = current_chat.username().expect("please set username in your telegram settings");

        let chat_id = current_chat.id.to_string();

        let state = State::from((username.to_string(), chat_id));

        let host = env::var("HOST").expect("HOST env variable is not set");
        let redirect_url = format!("https://{host}/webhook/token");

        let mut squard_connect_client = squard_connect_client.clone();

        let url_to_build = squard_connect_client.get_url::<State>(redirect_url, Some(state)).await?;

        let (network, public_key, max_epoch, randomness) = squard_connect_client.get_zk_proof_params();

        dialogue.update(LoginState::WalletParams(network, public_key, max_epoch, randomness)).await?;

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
            if let Some(call) = response.output.iter().find_map(|item| {
                if let OutputItem::FunctionCall(call) = item {
                    Some(call)
                } else {
                    None
                }
            }) {
                // Parse the arguments JSON
                let args: serde_json::Value =
                    serde_json::from_str(&call.arguments).unwrap_or_default();
                // Dispatch to our handlers
                let message = match call.name.as_str() {
                    "withdraw" => {
                        let amount = args.get("amount").and_then(|v| v.as_str()).unwrap_or("");
                        let coin = args.get("coin").and_then(|v| v.as_str()).unwrap_or("");
                        withdraw(&format!("withdraw {} {}", amount, coin))
                    }
                    "send" => {
                        let target = args.get("target").and_then(|v| v.as_str()).unwrap_or("");
                        let amount = args.get("amount").and_then(|v| v.as_str()).unwrap_or("");
                        let coin = args.get("coin").and_then(|v| v.as_str()).unwrap_or("");
                        send(&format!("send {} {} {}", target, amount, coin))
                    }
                    _ => "Unknown function call".to_string(),
                };
                let message = bot.send_message(msg.chat.id, message).await?;

                Ok(message)
            } else {
                // No function call: send direct model output
                let message = bot
                    .send_message(msg.chat.id, response.output_text())
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

pub async fn handle_get_balance(
    bot: Bot,
    msg: Message,
    dialogue: Dialogue<LoginState, InMemStorage<LoginState>>,
    squard_connect_client: SquardConnect,
) -> Result<Message, RequestError> {
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
            bot.send_message(msg.chat.id, format!("Your balance is {}", balance.total_balance)).await
        }
        None => {
            bot.send_message(msg.chat.id, "Error getting balance").await
        }
    }
}

pub async fn handle_get_wallet_address(
    bot: Bot,
    msg: Message,
    dialogue: Dialogue<LoginState, InMemStorage<LoginState>>,
    squard_connect_client: SquardConnect,
) -> Result<Message, RequestError> {
    let state = dialogue.get().await.unwrap();

    if let Some(LoginState::Authenticated(zk_login_inputs)) = state {
        let sender = squard_connect_client.get_sender(zk_login_inputs);
        bot.send_message(msg.chat.id, format!("Your wallet address is {}", sender.to_string())).await

    } else {
        bot.send_message(msg.chat.id, "Error getting wallet address").await
    }
}