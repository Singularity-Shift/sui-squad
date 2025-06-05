use crate::{
    bot_manage::{dto::BalanceObject, handler_tree::BotContext},
    middleware::{auth::auth, user::check_user},
    tools::{
        schema::get_schema,
        tools::{send_json, withdraw_json},
    },
};
use anyhow::Result as AnyhowResult;
use grammers_client::{
    types::{CallbackQuery, Message},
    InputMessage,
};
use reqwest::Url;

use std::{env, path::PathBuf};
use sui_sdk::{
    rpc_types::EventFilter,
    types::base_types::ObjectID,
};
use sui_squad_core::{
    commands::bot_commands::{Command, LoginState, UserId}, 
    package::dto::Event,
};

use super::dto::State;

// Main message handler - replaces teloxide dispatcher
pub async fn handle_message(ctx: BotContext, message: Message) -> AnyhowResult<()> {
    let _user_id = message.sender().unwrap().id();
    
    // Apply middleware filters
    if !auth(&ctx, &message).await? {
        return Ok(());
    }
    
    if !check_user(&ctx, &message).await? {
        return Ok(());
    }

    let text = message.text();
    
    // Parse command
    if let Some(command) = Command::parse(text) {
        handle_command(ctx, message, command).await?;
    }
    
    Ok(())
}

// Main callback query handler
pub async fn handle_callback_query(_ctx: BotContext, _query: CallbackQuery) -> AnyhowResult<()> {
    // Handle inline keyboard callbacks if needed
    Ok(())
}

// Command handler - delegates to answer.rs for proper separation of concerns
async fn handle_command(ctx: BotContext, message: Message, command: Command) -> AnyhowResult<()> {
    use crate::bot_manage::answer::answer;
    answer(&ctx, &message, command).await?;
    Ok(())
}

pub async fn handle_fund(
    ctx: BotContext,
    message: Message,
) -> AnyhowResult<()> {
    let chat = message.chat();
    let user_id = message.sender().unwrap().id();

    let mut squard_connect_client = ctx.squard_connect_client.clone();

    // Check if chat is a private chat (not group/channel)
    // In grammers, we need to check the chat type differently
    if let grammers_client::types::Chat::User(_) = chat {
        let user_id_str = user_id.to_string();

        let path_str = env::var("KEYSTORE_PATH").expect("PATH env variable is not set");

        let mut path = PathBuf::new();
        path.push(path_str);

        squard_connect_client.create_zkp_payload(path).await?;

        let (randomness, public_key, max_epoch) = squard_connect_client.get_zk_proof_params();

        let state = State::from((user_id_str.to_string(), max_epoch, public_key, randomness));

        let host = env::var("HOST").expect("HOST env variable is not set");
        let redirect_url = format!("https://{host}/webhook/token");

        let url_to_build = squard_connect_client
            .get_url::<State>(redirect_url, Some(state))
            .await?;

        let url = Url::parse(&url_to_build).unwrap();

        // Create inline keyboard with grammers using button and reply_markup
        use grammers_client::{button, reply_markup};
        
        let keyboard = reply_markup::inline(vec![
            vec![button::url("Fund your account", url.as_str())],
        ]);
        
        ctx.client
            .send_message(
                chat,
                InputMessage::text("Fund your account by signing in with your Google account")
                    .reply_markup(&keyboard),
            )
            .await?;
    } else {
        ctx.client
            .send_message(
                chat,
                InputMessage::text("You only can fund your account in private Bot chat"),
            )
            .await?;
    }

    Ok(())
}

pub async fn handle_prompt(
    ctx: BotContext,
    message: Message,
    prompt_text: String,
) -> AnyhowResult<()> {
    let user_id = message.sender().unwrap().id();
    let chat = message.chat();
    
    // Get user key for cache (user_id, chat_id)
    let user_key = (user_id.to_string(), chat.id().to_string());

    // Get cached conversation ID
    let previous_response_id = ctx.conversation_cache.get(&user_key).await;

    // Log conversation continuity status
    if let Some(ref prev_id) = previous_response_id {
        println!("🔗 Continuing conversation from: {}", prev_id);
    } else {
        println!("🆕 Starting new conversation");
    }

    // Define custom function/tool schemas for the model
    let schema = get_schema();

    // Call AI with function-calling enabled AND conversation continuity
    let mut current_response = ctx.responses_client
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
                    handle_get_balance_tool(ctx.clone(), user_id).await
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
        current_response = ctx.responses_client
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
    ctx.conversation_cache
        .update(user_key, current_response.id().to_string())
        .await;

    // Send final response
    let response_text = current_response.output_text();
    ctx.client
        .send_message(
            chat,
            InputMessage::text(response_text),
        )
        .await?;

    Ok(())
}

pub async fn handle_get_balance_tool(
    ctx: BotContext,
    user_id: UserId,
) -> String {
    // Get telegram_id from dialogue state
    let login_state = ctx.get_dialogue_state(user_id).await;
    if login_state.is_err() {
        return "Error: Unable to access user session".to_string();
    }

    let telegram_id_str = if let LoginState::LocalStorate(storage) = login_state.unwrap() {
        // Find any user's telegram_id from storage keys (UserId is telegram_id)
        if let Some(user_id_key) = storage.keys().next() {
            user_id_key.to_string()
        } else {
            return "Error: No user found in session".to_string();
        }
    } else {
        return "Error: User not logged in".to_string();
    };

    let node = ctx.squard_connect_client.get_node();

    let account_events = node
        .event_api()
        .query_events(
            EventFilter::MoveEventType(Event::AccountEvent.to_string().parse().unwrap()),
            None,
            None,
            false,
        )
        .await;

    if let Err(e) = account_events {
        return e.to_string();
    }

    let account_events_data = account_events.unwrap();
    let account_event = account_events_data.data.iter().find(|event| {
        if let Some(telegram_id) = event.parsed_json.get("telegram_id") {
            if let Some(event_telegram_id_str) = telegram_id.as_str() {
                return event_telegram_id_str == telegram_id_str;
            }
        }
        false
    });

    if account_event.is_none() {
        return "Account not found".to_string();
    }

    let account_event = account_event.unwrap();

    let account_id_value = account_event.parsed_json.get("account_id");

    if account_id_value.is_none() {
        return "Account id not found".to_string();
    }

    let account_id = account_id_value.unwrap().as_str();

    if account_id.is_none() {
        return "Account id not found".to_string();
    }

    let account_id = account_id.unwrap();

    let account_id_object_id = ObjectID::from_hex_literal(account_id);

    if account_id_object_id.is_err() {
        return "Account id not found".to_string();
    }

    let account_id_object_id = account_id_object_id.unwrap();

    let objects = node
        .read_api()
        .get_dynamic_fields(account_id_object_id, None, None)
        .await;

    if let Err(e) = objects {
        return e.to_string();
    }

    let objects = objects.unwrap();

    let object_info = objects.data.last();

    if object_info.is_none() {
        return "Object not found".to_string();
    }

    let object_info = object_info.unwrap();

    let name = object_info.name.clone();

    let object = node
        .read_api()
        .get_dynamic_field_object(account_id_object_id, name)
        .await;

    if let Err(e) = object {
        return e.to_string();
    }

    let object = object.unwrap().data;

    let object_data = object.unwrap();

    let object_data_content = object_data.content.unwrap();

    let balance_str = serde_json::to_string(&object_data_content);

    if balance_str.is_err() {
        return "Error: Unable to parse balance".to_string();
    }

    let balance_object = serde_json::from_str::<BalanceObject>(&balance_str.unwrap());

    if balance_object.is_err() {
        return "Error: Unable to parse balance".to_string();
    }

    let balance_object = balance_object.unwrap();

    let balance = balance_object.fields.value.fields.balance.parse::<u64>();

    if balance.is_err() {
        return "Error: Unable to parse balance".to_string();
    }

    let balance = balance.unwrap();

    // Convert from raw balance (with 9 decimals) to human-readable format
    let sui_decimals = 1_000_000_000u64; // 10^9
    let balance_in_sui = balance as f64 / sui_decimals as f64;

    // Format balance with appropriate decimal places
    let formatted_balance = if balance_in_sui == 0.0 {
        "0 SUI".to_string()
    } else if balance_in_sui < 0.001 {
        // Show more decimals for very small amounts
        format!("{:.9} SUI", balance_in_sui)
    } else if balance_in_sui < 1.0 {
        // Show 6 decimals for amounts less than 1 SUI
        format!("{:.6} SUI", balance_in_sui)
    } else {
        // Show 3 decimals for amounts 1 SUI and above
        format!("{:.3} SUI", balance_in_sui)
    };

    return formatted_balance;
}


