use teloxide::prelude::*;
use std::sync::Arc;
use bot_core::{
    config::Config,
    db::init_db,
    sui_gateway::DummyGateway,
    ai::OpenAIClient,
    commands::{admin, user},
};
use tracing_subscriber;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cfg = Config::from_env();
    let pool = init_db(&cfg.database_url).await?;
    let gateway = DummyGateway;
    let ai_client = OpenAIClient::new(cfg.openai_api_key.clone());
    let bot = Bot::new(cfg.telegram_bot_token.clone()).auto_send();

    teloxide::repl(bot.clone(), move |cx: UpdateWithCx<AutoSend<Bot>, Message>| {
        let gateway = gateway.clone();
        let ai_client = ai_client.clone();
        async move {
            if let Some(text) = cx.update.text() {
                // TODO: implement command parsing and dispatch to handlers
            }
        }
    })
    .await;

    Ok(())
}
