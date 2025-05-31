use std::path::PathBuf;

use squard_connect::client::squard_connect::SquardConnect;
use sui_sdk::types::base_types::SuiAddress;
use sui_squad_core::{commands::bot_commands::LoginState, helpers::dtos::UserPayload};
use teloxide::{
    Bot,
    dispatching::dialogue::InMemStorage,
    prelude::{Dialogue, Requester},
    types::Message,
};

use crate::onchain::account::create_account_if_not_exists;
use crate::services::services::Services;

pub async fn check_user(
    bot: Bot,
    msg: Message,
    dialogue: Dialogue<LoginState, InMemStorage<LoginState>>,
    squard_connect_client: SquardConnect,
    services: Services,
    path: PathBuf,
) {
    let mut wallet: Option<SuiAddress> = None;

    if msg.chat.is_group() {
        if let Ok(login_state) = dialogue.get().await {
            if let Some(LoginState::Authenticated(zk_login_inputs)) = login_state {
                wallet = Some(squard_connect_client.get_sender(zk_login_inputs));
            }
        };

        if let Some(wallet) = wallet {
            let user = UserPayload {
                telegram_id: msg.from().unwrap().id.to_string(),
                group_telegram_id: msg.chat.id.to_string(),
                wallet: wallet.to_string(),
            };

            let user_result = services.user(user).await;

            if let Err(e) = user_result {
                bot.send_message(msg.chat.id, format!("Error creating user: {}", e))
                    .await
                    .unwrap();
                return;
            }

            if let Ok(_user) = user_result {
                let node = squard_connect_client.get_node();

                let account_id = create_account_if_not_exists(wallet, path, node).await;

                if let Ok(account_id) = account_id {
                    let state = dialogue.update(LoginState::Account(account_id)).await;

                    if let Err(e) = state {
                        bot.send_message(msg.chat.id, format!("Error updating dialogue: {}", e))
                            .await
                            .unwrap();
                    }
                }
            }
        } else {
            bot.send_message(msg.chat.id, format!("You are not authenticated"))
                .await
                .unwrap();
        }
    }
}
