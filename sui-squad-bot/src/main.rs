mod bot_manage;
mod middleware;
mod services;
mod tools;

use anyhow::Result;
use bot_manage::handler_tree::BotContext;
use dotenvy::dotenv;
use grammers_client::{Client, Config as GrammersConfig, InitParams, Update};
use grammers_session::Session;
use services::services::Services;
use squard_connect::{client::squard_connect::SquardConnect, service::dtos::Network};
use std::time::Duration;
use std::{collections::HashMap, env, sync::Arc};
use sui_sdk::SuiClientBuilder;
use sui_squad_core::{
    ai::ResponsesClient, commands::bot_commands::{LoginState, UserId}, config::Config,
    conversation::ConversationCache,
};
use tokio::sync::Mutex;
use tracing_subscriber;
use grammers_client::grammers_tl_types as tl;

// Custom storage to replace teloxide's InMemStorage
pub type DialogueStorage = Arc<Mutex<HashMap<UserId, LoginState>>>;



#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let network_str: String =
        env::var("SUI_NETWORK").expect("SUI_NETWORK environment variable not set");

    let client_id =
        env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID environment variable not set");
    let api_key = env::var("ENOKI_API_KEY").expect("ENOKI_API_KEY environment variable not set");

    let network = match network_str.as_str() {
        "mainnet" => Network::Mainnet,
        "testnet" => Network::Testnet,
        _ => Network::Devnet,
    };

    let node = match network {
        Network::Mainnet => SuiClientBuilder::default().build_mainnet().await?,
        Network::Testnet => SuiClientBuilder::default().build_testnet().await?,
        _ => SuiClientBuilder::default().build_devnet().await?,
    };

    let squard_connect_client = SquardConnect::new(node, client_id, network, api_key);

    let services = Services::new();

    // Create conversation cache with 10-minute TTL
    let conversation_cache = ConversationCache::new(Duration::from_secs(600));
    let cache_for_cleanup = conversation_cache.clone();

    // Spawn cleanup task that runs every minute
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(60));
        loop {
            interval.tick().await;
            cache_for_cleanup.cleanup_expired().await;
            println!("🧹 Cleaned up expired conversations");
        }
    });

    println!("✅ Conversation cache initialized with 10-minute TTL");

    tracing_subscriber::fmt::init();

    let cfg = Config::from_env();
    let responses_client = ResponsesClient::new(&cfg)?;
    
    // Initialize grammers client using config values
    let session = Session::new();
    
    let client = Client::connect(GrammersConfig {
        session,
        api_id: cfg.api_id,
        api_hash: cfg.api_hash.clone(),
        params: InitParams {
            server_addr: Some("149.154.167.92:443".parse()?), // DC 4 - US East Coast
            device_model: "sui-squad-bot".to_string(),
            system_version: "1.0".to_string(),
            app_version: "1.0".to_string(),
            lang_code: "en".to_string(),
            system_lang_code: "en".to_string(),
            catch_up: false,
            flood_sleep_threshold: 60,
            ..Default::default()
        },
    }).await?;

    // Bot authentication with grammers
    if !client.is_authorized().await? {
        println!("🔐 Bot not authorized, signing in...");
        
        // Use the bot token from config for authentication
        let bot_token = &cfg.bot_token;
        
        // Implement proper bot authentication with grammers using ImportBotAuthorization
        use tl::functions;

        let import_req = functions::auth::ImportBotAuthorization {
            flags: 0,
            api_id: cfg.api_id,
            api_hash: cfg.api_hash.clone(),
            bot_auth_token: bot_token.clone(),
        };

        // Try to authenticate - grammers client should handle DC migration automatically
        let auth_result = client.invoke(&import_req).await;

        match auth_result {
            Ok(_) => println!("✅ Bot authentication successful"),
            Err(e) => {
                eprintln!("❌ Bot authentication failed: {}", e);
                return Err(e.into());
            }
        }
    } else {
        println!("✅ Bot already authorized");
    }

    println!("✅ Bot connected successfully");

    // Create in-memory storage replacement
    let dialogue_storage: DialogueStorage = Arc::new(Mutex::new(HashMap::new()));
    let hash_map: HashMap<UserId, String> = HashMap::new();

    // Create bot context with all dependencies
    let bot_context = BotContext {
        client: client.clone(),
        responses_client,
        dialogue_storage,
        squard_connect_client,
        services,
        conversation_cache,
        user_sessions: Arc::new(Mutex::new(hash_map)),
    };

    // Start the event loop
    println!("🤖 Starting bot event loop...");
    
    loop {
        match client.next_update().await {
            Ok(update) => {
                let ctx = bot_context.clone();
                tokio::spawn(async move {
                    if let Err(e) = handle_update(ctx, update).await {
                        eprintln!("Error handling update: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Error receiving update: {}", e);
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

async fn handle_update(ctx: BotContext, update: Update) -> Result<()> {
    match update {
        Update::NewMessage(message) => {
            bot_manage::handlers::handle_message(ctx, message).await?;
        }
        Update::CallbackQuery(query) => {
            bot_manage::handlers::handle_callback_query(ctx, query).await?;
        }
        _ => {
            // Handle other update types if needed
        }
    }
    Ok(())
}
