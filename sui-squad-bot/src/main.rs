use teloxide::prelude::*;
use sui_squad_core::{
    config::Config,
    db::init_db,
    sui_gateway::DummyGateway,
    commands::{admin, user},
};
use sui_squad_core::ai::openai_client::OpenAIClient;
use tracing_subscriber;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cfg = Config::from_env();
    let pool = init_db(&cfg.database_url).await?;
    let gateway = DummyGateway;
    let ai_client = OpenAIClient::new(cfg.openai_api_key.clone());
    let bot = Bot::new(cfg.teloxide_token.clone());

    Dispatcher::builder(
        bot.clone(),
        Update::filter_message().endpoint(|cx: Message, _bot: Bot| async move {
            if let Some(text) = cx.text() {
                println!("Received: {}", text);
            }
            Ok(()) as Result<(), Box<dyn std::error::Error + Send + Sync>>
        })
    )
    .build()
    .dispatch()
    .await;

    Ok(())
}
