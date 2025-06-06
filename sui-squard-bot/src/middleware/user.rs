use sled::Db;
use sui_squad_core::{commands::bot_commands::LoginState, helpers::dtos::Storage};
use teloxide::{dispatching::dialogue::InMemStorage, prelude::Dialogue, types::Message};
use tracing::{debug, error, info, warn};

use crate::services::services::Services;

pub async fn check_user(
    msg: Message,
    dialogue: Dialogue<LoginState, InMemStorage<LoginState>>,
    services: Services,
    db: Db,
) -> bool {
    let user_id = msg.from.clone().map(|u| u.id);
    debug!("🔍 Starting user check for user_id: {:?}", user_id);

    let login_state = dialogue.get().await;

    if login_state.is_err() {
        error!("❌ Failed to get dialogue state: {:?}", login_state.err());
        return false;
    }

    if let Some(login_state) = login_state.unwrap() {
        debug!(
            "📋 Login state retrieved successfully: {:?}",
            std::mem::discriminant(&login_state)
        );

        if let LoginState::LocalStorate(storage) = login_state {
            debug!("📦 Found LocalStorage with {} entries", storage.len());

            let telegram_id = msg.from;

            if telegram_id.is_some() {
                let user = telegram_id.unwrap();
                debug!("👤 Processing user check for telegram_id: {}", user.id);

                let result = storage.get(&user.id);

                if result.is_some() {
                    debug!("✅ Storage entry found for user: {}", user.id);

                    let storage_json = result.unwrap();
                    debug!("📄 Storage JSON length: {} characters", storage_json.len());

                    match serde_json::from_str::<Storage>(&storage_json) {
                        Ok(storage) => {
                            debug!("✅ Successfully parsed storage JSON for user: {}", user.id);
                            debug!("🔑 JWT token present: {}", !storage.jwt.is_empty());

                            info!("🌐 Making user service call for user: {}", user.id);
                            let response = services.user(storage.jwt.clone()).await;

                            if response.is_ok() {
                                info!("✅ User service call successful for user: {}", user.id);

                                if let Some(username) = user.username {
                                    let entity = db.get(username.as_bytes());

                                    if entity.is_ok() {
                                        let entity_vec = entity.unwrap();

                                        if entity_vec.is_some() {
                                            let entity_str =
                                                serde_json::from_slice::<u64>(&entity_vec.unwrap());

                                            if entity_str.is_ok() {
                                                let entity_str = entity_str.unwrap();
                                                info!(
                                                    "✅ Username found in database: {}",
                                                    entity_str
                                                );
                                                return true;
                                            } else {
                                                error!(
                                                    "❌ Failed to convert username to string: {}",
                                                    entity_str.err().unwrap()
                                                );
                                                return false;
                                            }
                                        } else {
                                            warn!(
                                                "❌ Username not found in database: {}",
                                                username
                                            );
                                            return false;
                                        }
                                    } else {
                                        let db_result = db.insert(
                                            username.clone(),
                                            user.id.to_string().as_bytes(),
                                        );

                                        if db_result.is_ok() {
                                            info!("✅ Username added to database: {}", username);
                                            return true;
                                        } else {
                                            error!(
                                                "❌ Failed to add username to database: {}",
                                                username
                                            );
                                            return false;
                                        }
                                    }
                                } else {
                                    error!(
                                        "❌ username is empty but required to use the bot: {}",
                                        user.id
                                    );
                                    return false;
                                }
                            } else {
                                let error_details = response.err().unwrap();
                                error!("❌ User service call failed for user: {}", user.id);
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
                            error!("❌ Failed to parse storage JSON for user: {}", user.id);
                            error!("❌ JSON parse error: {:?}", parse_error);
                            error!("❌ Raw JSON content: {}", storage_json);
                        }
                    }
                } else {
                    warn!("⚠️ No storage entry found for user: {}", user.id);
                    debug!(
                        "📦 Available storage keys: {:?}",
                        storage.keys().collect::<Vec<_>>()
                    );
                }
            } else {
                warn!("⚠️ No telegram user found in message");
                debug!(
                    "📨 Message details - chat_id: {}, message_id: {}",
                    msg.chat.id, msg.id
                );
            }

            debug!("❌ User check failed for LocalStorage state");
            return false;
        } else {
            warn!(
                "⚠️ Login state is not LocalStorage type: {:?}",
                std::mem::discriminant(&login_state)
            );
            return false;
        }
    } else {
        warn!("⚠️ No login state found in dialogue");
        return false;
    }
}
