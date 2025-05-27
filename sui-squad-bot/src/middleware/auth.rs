use squard_connect::client::squard_connect::SquardConnect;
use sui_squad_core::{commands::bot_commands::LoginState, helpers::dtos::AuthRequest};
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*, types::Message};

use crate::services::services::Services;

pub async fn auth(msg: Message, dialogue: Dialogue<LoginState, InMemStorage<LoginState>>, squard_connect_client: SquardConnect, services: Services) -> bool {
    let login_state = dialogue.get().await; 

    if let Ok(ref login_state_option) = login_state {
        if let Some(login_state) = login_state_option {
            if let LoginState::Authenticated(_seed_address) = login_state {
                return true;
            }
        }
    } else {
        println!("Error getting dialogue state: {:?}", login_state.as_ref().err());
    }
    
    let auth_request = AuthRequest {
        chat_id: msg.chat.id.to_string(),
        username: msg.chat.username().unwrap_or_default().to_string(),
    };
    
    let user = services.auth(auth_request).await;

    if let Ok(user) = user {
        let mut squard_connect_client = squard_connect_client.clone();
        squard_connect_client.set_jwt(user.jwt.clone());

        if let LoginState::WalletParams(network, public_key, max_epoch, randomness) = login_state.as_ref().unwrap().as_ref().unwrap(){
            squard_connect_client.set_zk_proof_params(network.clone(), public_key.clone(), *max_epoch, randomness.clone());
        }else {
            println!("Error getting wallet params");
            return false;
        }
        
        let zk_login_inputs_result = squard_connect_client.recover_seed_address().await;

        if let Ok(zk_loing_inputs) = zk_login_inputs_result {
            let state = dialogue.update(LoginState::Authenticated(zk_loing_inputs)).await;

            if let Err(e) = state {
                println!("Error updating dialogue: {:?}", e);
                return false;
            }
            return true;
        } else {
            println!("Error recovering seed address: {:?}", zk_login_inputs_result.err());
            return false;
        }
    } else {
        println!("Error authenticating user: {:?}", user.err());
        return false;
    }
}