use sled::Db;
use sui_squad_core::helpers::jwt::JwtManager;
use teloxide::{prelude::*, types::Message};

use crate::credentials::dto::Credentials;
use crate::credentials::helpers::{get_credentials, save_credentials};

pub async fn auth(msg: Message, db: Db) -> bool {
    let jwt_manager = JwtManager::new();

    let user = msg.from;

    if user.is_none() {
        println!("❌ User not found");
        return false;
    }

    let user = user.unwrap();

    let username = user.username;

    if username.is_none() {
        println!("❌ Username not found");
        return false;
    }

    let username = username.unwrap();

    let credentials_opt = get_credentials(&username, db.clone());

    if let Some(credentials) = credentials_opt {
        // Initialize JWT manager and validate/update storage
        match jwt_manager.validate_and_update_jwt(credentials.jwt, credentials.user_id) {
            Ok(_updated_storage) => {
                println!("✅ JWT token validated/generated for user {}", user.id);
                // Note: The updated storage with the new JWT would need to be
                // persisted back to the dialogue storage in the calling code
                return true;
            }
            Err(e) => {
                println!("❌ Failed to validate/generate JWT: {}", e);
            }
        }

        return generate_new_jwt(username, user.id, jwt_manager, db).await;
    }

    println!("❌ No credentials found for user {}", username);
    return generate_new_jwt(username, user.id, jwt_manager, db).await;
}

async fn generate_new_jwt(
    username: String,
    user_id: UserId,
    jwt_manager: JwtManager,
    db: Db,
) -> bool {
    match jwt_manager.generate_token(user_id) {
        Ok(token) => {
            let jwt = token;

            let credentials = Credentials::from((jwt, user_id));

            let saved = save_credentials(&username, credentials, db);

            if saved.is_err() {
                println!("❌ Failed to save credentials: {}", saved.err().unwrap());
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
