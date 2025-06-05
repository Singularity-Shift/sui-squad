use std::collections::HashMap;
use sui_squad_core::helpers::jwt::JwtManager;
use sui_squad_core::{commands::bot_commands::LoginState, helpers::dtos::Storage};
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*, types::Message};

pub async fn auth(msg: Message, dialogue: Dialogue<LoginState, InMemStorage<LoginState>>) -> bool {
    let login_state = dialogue.get().await;
    let jwt_manager = JwtManager::new();

    if let Ok(ref login_state_option) = login_state {
        if let Some(login_state) = login_state_option {
            let user_opt = msg.from();

            if let Some(user) = user_opt {
                if let LoginState::LocalStorate(users) = login_state {
                    if let Some(storage_str) = users.get(&user.id) {
                        let storage_result: Result<_, serde_json::Error> =
                            serde_json::from_str::<Storage>(storage_str);

                        if let Ok(storage) = storage_result {
                            // Initialize JWT manager and validate/update storage

                            match jwt_manager.validate_and_update_storage(storage, user.id) {
                                Ok(_updated_storage) => {
                                    println!(
                                        "✅ JWT token validated/generated for user {}",
                                        user.id
                                    );
                                    // Note: The updated storage with the new JWT would need to be
                                    // persisted back to the dialogue storage in the calling code
                                    return true;
                                }
                                Err(e) => {
                                    println!("❌ Failed to validate/generate JWT: {}", e);
                                }
                            }

                            return generate_new_storage(
                                user.id,
                                jwt_manager,
                                users.clone(),
                                dialogue,
                            )
                            .await;
                        }
                    } else {
                        return generate_new_storage(user.id, jwt_manager, users.clone(), dialogue)
                            .await;
                    }
                } else {
                    let users = HashMap::new();
                    return generate_new_storage(user.id, jwt_manager, users, dialogue).await;
                }
            }
        }
    } else {
        println!(
            "Error getting dialogue state: {:?}",
            login_state.as_ref().err()
        );

        return false;
    }

    false
}

async fn generate_new_storage(
    user_id: UserId,
    jwt_manager: JwtManager,
    mut users: HashMap<UserId, String>,
    dialogue: Dialogue<LoginState, InMemStorage<LoginState>>,
) -> bool {
    match jwt_manager.generate_token(user_id) {
        Ok(token) => {
            let storage = Storage { jwt: token };

            let storage_str = serde_json::to_string(&storage).unwrap();

            users.insert(user_id, storage_str);

            let new_login_state = LoginState::LocalStorate(users);

            if let Err(e) = dialogue.update(new_login_state.clone()).await {
                println!("❌ Failed to update dialogue state: {}", e);
                return false;
            }

            println!("✅ Generated new JWT token for user {}", user_id);
            return true;
        }
        Err(e) => {
            println!("❌ Failed to generate JWT token: {}", e);
            return false;
        }
    }
}
