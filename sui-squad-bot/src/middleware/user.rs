use sui_squad_core::{commands::bot_commands::LoginState, helpers::dtos::Storage};
use grammers_client::types::Message;
use tracing::{debug, error, info, warn};
use anyhow::Result;
use crate::bot_manage::handler_tree::BotContext;

pub async fn check_user(
    ctx: &BotContext,
    message: &Message,
) -> Result<bool> {
    let user_id = message.sender().unwrap().id();
    debug!("🔍 Starting user check for user_id: {}", user_id);

    let login_state = ctx.get_dialogue_state(user_id).await;

    if login_state.is_err() {
        error!("❌ Failed to get dialogue state: {:?}", login_state.as_ref().err());
        return Ok(false);
    }

    let login_state = login_state.unwrap();
    debug!(
        "📋 Login state retrieved successfully: {:?}",
        std::mem::discriminant(&login_state)
    );

    match login_state {
        LoginState::LocalStorate(storage) => {
            debug!("📦 Found LocalStorage with {} entries", storage.len());

            debug!("👤 Processing user check for telegram_id: {}", user_id);

            let result = storage.get(&user_id);

            if result.is_some() {
                debug!("✅ Storage entry found for user: {}", user_id);

                let storage_json = result.unwrap();
                debug!("📄 Storage JSON length: {} characters", storage_json.len());

                match serde_json::from_str::<Storage>(&storage_json) {
                    Ok(storage) => {
                        debug!("✅ Successfully parsed storage JSON for user: {}", user_id);
                        debug!("🔑 JWT token present: {}", !storage.jwt.is_empty());

                        info!("🌐 Making user service call for user: {}", user_id);
                        let response = ctx.services.user(storage.jwt.clone()).await;

                        if response.is_ok() {
                            info!("✅ User service call successful for user: {}", user_id);
                            return Ok(true);
                        } else {
                            let error_details = response.err().unwrap();
                            error!("❌ User service call failed for user: {}", user_id);
                            error!("❌ Service error details: {:?}", error_details);
                            error!(
                                "❌ JWT token (first 20 chars): {}...",
                                if storage.jwt.len() > 20 {
                                    &storage.jwt[..20]
                                } else {
                                    &storage.jwt
                                }
                            );

                            // Log additional context
                            warn!(
                                "⚠️ User authentication failed - JWT may be expired or invalid"
                            );
                        }
                    }
                    Err(parse_error) => {
                        error!("❌ Failed to parse storage JSON for user: {}", user_id);
                        error!("❌ JSON parse error: {:?}", parse_error);
                        error!("❌ Raw JSON content: {}", storage_json);
                    }
                }
            } else {
                warn!("⚠️ No storage entry found for user: {}", user_id);
                debug!(
                    "📦 Available storage keys: {:?}",
                    storage.keys().collect::<Vec<_>>()
                );
            }

            debug!("❌ User check failed for LocalStorage state");
            return Ok(false);
        }
        LoginState::Login => {
            warn!("⚠️ Login state is Login type, user not authenticated");
            return Ok(false);
        }
    }
}
