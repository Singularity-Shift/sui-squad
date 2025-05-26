use squard_connect::client::squard_connect::SquardConnect;
use sui_squad_core::{commands::bot_commands::LoginState, helpers::dtos::AuthRequest};
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*, types::Message};

use crate::services::services::Services;

pub async fn auth(msg: Message, dialogue: Dialogue<LoginState, InMemStorage<LoginState>>, squard_connect_client: SquardConnect, services: Services) -> bool {
    let auth_request = AuthRequest {
        chat_id: msg.chat.id.to_string(),
        username: msg.chat.username().unwrap_or_default().to_string(),
    };
    
    let user = services.auth(auth_request).await;

    if let Ok(user) = user {
        squard_connect_client.clone().set_jwt(user.jwt.clone());
        let seed_address = squard_connect_client.recover_seed_address().await;

        if let Ok(seed_address) = seed_address {
            let state = dialogue.update(LoginState::Authenticated(seed_address)).await;

            if let Err(e) = state {
                println!("Error updating dialogue: {:?}", e);
                return false;
            }
            return true;
        } else {
            println!("Error recovering seed address");
            return false;
        }
    } else {
        println!("Error authenticating user");
        return false;
    }
}