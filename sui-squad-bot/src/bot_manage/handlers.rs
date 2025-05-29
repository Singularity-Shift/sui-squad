use crate::tools::{
    schema::get_schema,
    tools::{send_json, withdraw_json},
};
use anyhow::Result as AnyhowResult;
use reqwest::Url;
use squard_connect::client::squard_connect::SquardConnect;
use std::env;
use sui_squad_core::{
    ai::ResponsesClient, 
    commands::bot_commands::LoginState, 
    conversation::ConversationCache,
};
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
    conversation_cache: ConversationCache,
) -> AnyhowResult<Message> {
    // Get user key for cache (user_id, chat_id)
    let user_key = (
        msg.from().unwrap().id.to_string(), 
        msg.chat.id.to_string()
    );
    
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
        .generate_with_tools_continuous(&prompt_text, schema.clone(), previous_response_id)
        .await?;
    
    let mut iteration = 1;
    const MAX_ITERATIONS: usize = 5; // Prevent infinite loops
    
    // Handle function calling loop
    while !current_response.tool_calls().is_empty() && iteration <= MAX_ITERATIONS {
        println!("🔧 Iteration {}: Processing {} tool calls", 
                iteration, current_response.tool_calls().len());
        
        let mut function_outputs = Vec::new();
        
        // Process all tool calls
        for tool_call in current_response.tool_calls() {
            println!("   📞 Function: {} ({})", tool_call.name, tool_call.call_id);
            println!("   📋 Arguments: {}", tool_call.arguments);
            
            // Execute function based on name
            let result = match tool_call.name.as_str() {
                "get_wallet" => {
                    handle_get_wallet_tool(dialogue.clone(), squard_connect_client.clone()).await
                }
                "get_balance" => {
                    let args: serde_json::Value = serde_json::from_str(&tool_call.arguments)
                        .unwrap_or_else(|_| serde_json::json!({}));
                    let token = args.get("token").and_then(|v| v.as_str()).map(String::from);
                    handle_get_balance_tool(dialogue.clone(), squard_connect_client.clone(), token).await
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
        
        // Submit tool outputs and get next response
        current_response = responses_client
            .submit_tool_outputs(
                current_response.id().to_string(), 
                function_outputs, 
                schema.clone()
            )
            .await?;
        
        iteration += 1;
    }
    
    if iteration > MAX_ITERATIONS {
        println!("⚠️ Stopped after {} iterations to prevent infinite loop", MAX_ITERATIONS);
    }
    
    // Update cache with new response ID for next turn
    conversation_cache.update(user_key, current_response.id().to_string()).await;
    
    // Send final response
    let response_text = current_response.output_text();
    let message = bot
        .send_message(msg.chat.id, response_text)
        .parse_mode(ParseMode::Html)
        .await?;
    
    Ok(message)
}

pub async fn handle_get_wallet_tool(
    dialogue: Dialogue<LoginState, InMemStorage<LoginState>>,
    squard_connect_client: SquardConnect,
) -> String {
    let state = dialogue.get().await.unwrap();

    if let Some(LoginState::Authenticated(zk_login_inputs)) = state {
        let sender = squard_connect_client.get_sender(zk_login_inputs);
        format!("Your wallet address is:\n<code>{}</code>", sender.to_string())
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