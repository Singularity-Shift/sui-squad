use std::collections::HashMap;
use sui_squad_core::helpers::jwt::JwtManager;
use sui_squad_core::{commands::bot_commands::{LoginState, UserId}, helpers::dtos::Storage};
use grammers_client::types::Message;
use anyhow::Result;
use crate::bot_manage::handler_tree::BotContext;

pub async fn auth(ctx: &BotContext, message: &Message) -> Result<bool> {
    let user_id = message.sender().unwrap().id();
    let login_state = ctx.get_dialogue_state(user_id).await?;
    let jwt_manager = JwtManager::new();

    match login_state {
        LoginState::LocalStorate(users) => {
            if let Some(storage_str) = users.get(&user_id) {
                let storage_result: Result<Storage, serde_json::Error> =
                    serde_json::from_str::<Storage>(storage_str);

                if let Ok(storage) = storage_result {
                    // Initialize JWT manager and validate/update storage
                    match jwt_manager.validate_and_update_storage(storage, user_id) {
                        Ok(_updated_storage) => {
                            println!(
                                "✅ JWT token validated/generated for user {}",
                                user_id
                            );
                            // Note: The updated storage with the new JWT would need to be
                            // persisted back to the dialogue storage in the calling code
                            return Ok(true);
                        }
                        Err(e) => {
                            println!("❌ Failed to validate/generate JWT: {}", e);
                        }
                    }

                    return generate_new_storage(
                        user_id,
                        jwt_manager,
                        users.clone(),
                        ctx,
                    )
                    .await;
                }
            } else {
                return generate_new_storage(user_id, jwt_manager, users.clone(), ctx).await;
            }
        }
        LoginState::Login => {
            let users = HashMap::new();
            return generate_new_storage(user_id, jwt_manager, users, ctx).await;
        }
    }

    Ok(false)
}

async fn generate_new_storage(
    user_id: UserId,
    jwt_manager: JwtManager,
    mut users: HashMap<UserId, String>,
    ctx: &BotContext,
) -> Result<bool> {
    match jwt_manager.generate_token(user_id) {
        Ok(token) => {
            let storage = Storage { jwt: token };

            let storage_str = serde_json::to_string(&storage).unwrap();

            users.insert(user_id, storage_str);

            let new_login_state = LoginState::LocalStorate(users);

            if let Err(e) = ctx.set_dialogue_state(user_id, new_login_state).await {
                println!("❌ Failed to update dialogue state: {}", e);
                return Ok(false);
            }

            println!("✅ Generated new JWT token for user {}", user_id);
            return Ok(true);
        }
        Err(e) => {
            println!("❌ Failed to generate JWT token: {}", e);
            return Ok(false);
        }
    }
}
