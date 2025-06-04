use crate::tools::{
    schema::get_schema,
    tools::{send_json, withdraw_json},
};
use anyhow::Result as AnyhowResult;
use reqwest::Url;
use squard_connect::client::squard_connect::SquardConnect;
use std::{env, path::PathBuf};
use sui_squad_core::{
    ai::ResponsesClient, commands::bot_commands::LoginState, conversation::ConversationCache,
};
use teloxide::{
    Bot,
    dispatching::dialogue::InMemStorage,
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Me, Message, ParseMode},
};

use super::dto::State;

pub async fn handle_fund(
    bot: Bot,
    msg: Message,
    squard_connect_client: SquardConnect,
) -> AnyhowResult<Message> {
    let current_chat = msg.chat.clone();
    let message: Message;

    let mut squard_connect_client = squard_connect_client.clone();

    if !current_chat.is_group() {
        let user_id = msg.from().unwrap().id.to_string();

        let path_str = env::var("KEYSTORE_PATH").expect("PATH env variable is not set");

        let mut path = PathBuf::new();

        path.push(path_str);

        squard_connect_client.create_zkp_payload(path).await?;

        let max_epoch = squard_connect_client.get_max_epoch();

        let public_key = squard_connect_client.get_public_key();

        let state = State::from((user_id.to_string(), max_epoch, public_key));

        let host = env::var("HOST").expect("HOST env variable is not set");
        let redirect_url = format!("https://{host}/webhook/token");

        let url_to_build = squard_connect_client
            .get_url::<State>(redirect_url, Some(state))
            .await?;

        let url = Url::parse(&url_to_build).unwrap();

        let fund_button = vec![vec![InlineKeyboardButton::new(
            "Fund your account",
            teloxide::types::InlineKeyboardButtonKind::Url(url),
        )]];

        let markdown = InlineKeyboardMarkup::new(fund_button);

        message = bot
            .send_message(
                current_chat.id,
                "Fund your account by signing in with your Google account",
            )
            .reply_markup(markdown)
            .await?;
    } else {
        message = bot
            .send_message(
                current_chat.id,
                "You only can fund your account in private Bot chat",
            )
            .await?;
    }

    Ok(message)
}

pub async fn handle_prompt(
    bot: Bot,
    msg: Message,
    prompt_text: String,
    responses_client: ResponsesClient,
    dialogue: Dialogue<LoginState, InMemStorage<LoginState>>,
    squard_connect_client: SquardConnect,
    conversation_cache: ConversationCache,
) -> AnyhowResult<Message> {
    // Get user key for cache (user_id, chat_id)
    let user_key = (msg.from().unwrap().id.to_string(), msg.chat.id.to_string());

    // Get cached conversation ID
    let previous_response_id = conversation_cache.get(&user_key).await;

    // Log conversation continuity status
    if let Some(ref prev_id) = previous_response_id {
        println!("🔗 Continuing conversation from: {}", prev_id);
    } else {
        println!("🆕 Starting new conversation");
    }

    // Define custom function/tool schemas for the model
    let schema = get_schema();

    // Call AI with function-calling enabled AND conversation continuity
    let mut current_response = responses_client
        .generate_response(
            Some(&prompt_text),
            Some(schema.clone()),
            previous_response_id,
            None,
        )
        .await?;

    let mut iteration = 1;
    const MAX_ITERATIONS: usize = 5; // Prevent infinite loops

    // Handle function calling loop
    while !current_response.tool_calls().is_empty() && iteration <= MAX_ITERATIONS {
        println!(
            "🔧 Iteration {}: Processing {} tool calls",
            iteration,
            current_response.tool_calls().len()
        );

        let mut function_outputs = Vec::new();

        // Process all tool calls
        for tool_call in current_response.tool_calls() {
            println!("   📞 Function: {} ({})", tool_call.name, tool_call.call_id);
            println!("   📋 Arguments: {}", tool_call.arguments);

            // Execute function based on name
            let result = match tool_call.name.as_str() {
                "get_balance" => {
                    let args: serde_json::Value = serde_json::from_str(&tool_call.arguments)
                        .unwrap_or_else(|_| serde_json::json!({}));
                    let token = args.get("token").and_then(|v| v.as_str()).map(String::from);
                    handle_get_balance_tool(dialogue.clone(), squard_connect_client.clone(), token)
                        .await
                }
                "withdraw" => {
                    let args: serde_json::Value = serde_json::from_str(&tool_call.arguments)
                        .unwrap_or_else(|_| serde_json::json!({}));
                    withdraw_json(&args)
                }
                "send" => {
                    let args: serde_json::Value = serde_json::from_str(&tool_call.arguments)
                        .unwrap_or_else(|_| serde_json::json!({}));
                    send_json(&args)
                }
                _ => format!("Unknown function call: {}", tool_call.name),
            };

            println!("   ✅ Result: {}", result);
            function_outputs.push((tool_call.call_id.clone(), result));
        }

        // Submit tool outputs and get next response using unified method
        current_response = responses_client
            .generate_response(
                None,
                Some(schema.clone()),
                None,
                Some((current_response.id().to_string(), function_outputs)),
            )
            .await?;

        iteration += 1;
    }

    if iteration > MAX_ITERATIONS {
        println!(
            "⚠️ Stopped after {} iterations to prevent infinite loop",
            MAX_ITERATIONS
        );
    }

    // Update cache with new response ID for next turn
    conversation_cache
        .update(user_key, current_response.id().to_string())
        .await;

    // Send final response
    let response_text = current_response.output_text();
    let message = bot
        .send_message(msg.chat.id, response_text)
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(message)
}

pub async fn handle_get_balance_tool(
    dialogue: Dialogue<LoginState, InMemStorage<LoginState>>,
    squard_connect_client: SquardConnect,
    token: Option<String>,
) -> String {
    "Balance functionality not implemented yet".to_string()
}
