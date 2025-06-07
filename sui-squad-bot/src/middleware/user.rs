use sled::Db;
use teloxide::types::Message;
use tracing::{debug, error};

use crate::{credentials::helpers::get_credentials, services::services::Services};

pub async fn check_user(msg: Message, services: Services, db: Db) -> bool {
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

    debug!("🔍 Starting user check for username: {:?}", username);

    let credentials_opt = get_credentials(&username, db.clone());

    if let Some(credentials) = credentials_opt {
        let jwt = credentials.jwt;

        let response = services.user(jwt).await;

        if response.is_err() {
            error!("❌ Failed to get user: {:?}", response.err());
            return false;
        } else {
            println!("✅ User found: {:?}", username);
        }
    }

    return true;
}
