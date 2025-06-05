use anyhow::Result;
use grammers_client::Client;
use std::{collections::HashMap, sync::Arc};
use sui_squad_core::{
    ai::ResponsesClient,
    commands::bot_commands::{LoginState, UserId},
    conversation::ConversationCache,
};
use squard_connect::client::squard_connect::SquardConnect;
use tokio::sync::Mutex;
use crate::services::services::Services;

pub type DialogueStorage = Arc<Mutex<HashMap<UserId, LoginState>>>;
pub type UserSessions = Arc<Mutex<HashMap<UserId, String>>>;

#[derive(Clone)]
pub struct BotContext {
    pub client: Client,
    pub responses_client: ResponsesClient,
    pub dialogue_storage: DialogueStorage,
    pub squard_connect_client: SquardConnect,
    pub services: Services,
    pub conversation_cache: ConversationCache,
    pub user_sessions: UserSessions,
}

impl BotContext {
    pub async fn get_dialogue_state(&self, user_id: UserId) -> Result<LoginState> {
        let storage = self.dialogue_storage.lock().await;
        Ok(storage.get(&user_id).cloned().unwrap_or_default())
    }

    pub async fn set_dialogue_state(&self, user_id: UserId, state: LoginState) -> Result<()> {
        let mut storage = self.dialogue_storage.lock().await;
        storage.insert(user_id, state);
        Ok(())
    }

    pub async fn get_user_session(&self, user_id: UserId) -> Result<Option<String>> {
        let sessions = self.user_sessions.lock().await;
        Ok(sessions.get(&user_id).cloned())
    }

    pub async fn set_user_session(&self, user_id: UserId, session: String) -> Result<()> {
        let mut sessions = self.user_sessions.lock().await;
        sessions.insert(user_id, session);
        Ok(())
    }
}
