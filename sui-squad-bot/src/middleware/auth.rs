use squard_connect::{client::squard_connect::SquardConnect, service::dtos::Network};
use sui_squad_core::{commands::bot_commands::LoginState, helpers::dtos::AuthRequest};
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*, types::Message};

use crate::services::services::Services;

pub async fn auth(
    bot: Bot,
    msg: Message,
    dialogue: Dialogue<LoginState, InMemStorage<LoginState>>,
    squard_connect_client: SquardConnect,
    services: Services,
) -> bool {
    let auth_request = AuthRequest {
        bot_id: bot.get_me().await.unwrap().id.to_string(),
        user_id: msg.from().unwrap().id.to_string(),
    };

    println!("Auth request: {:?}", auth_request);

    let user = services.auth(auth_request).await;

    if let Ok(user) = user {
        let mut squard_connect_client = squard_connect_client.clone();
        squard_connect_client.set_jwt(user.jwt.clone());

        squard_connect_client.set_zk_proof_params(
            Network::from(user.network),
            user.public_key,
            user.max_epoch,
            user.randomness,
        );

        let zk_login_inputs_result = squard_connect_client.recover_seed_address().await;

        if let Ok(zk_loing_inputs) = zk_login_inputs_result {
            let state = dialogue
                .update(LoginState::Authenticated(zk_loing_inputs))
                .await;

            if let Err(e) = state {
                println!("Error updating dialogue: {:?}", e);
                return false;
            }
            return true;
        } else {
            println!(
                "Error recovering seed address: {:?}",
                zk_login_inputs_result.err()
            );
            return false;
        }
    } else {
        println!("Error authenticating user: {:?}", user.err());
        return false;
    }
}
