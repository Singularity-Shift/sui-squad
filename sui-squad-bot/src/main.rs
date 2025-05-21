mod bot_manage;
mod tools;

use bot_manage::handler_tree::handler_tree;

use anyhow::Result;
use dotenvy::dotenv;
use squard_connect::{client::squard_connect::SquardConnect, service::dtos::Network};
use std::env;
use sui_sdk::SuiClientBuilder;
use sui_squad_core::{ai::ResponsesClient, commands::bot_commands::LoginState, config::Config};
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*, types::BotCommand};
use tracing_subscriber;

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

    tracing_subscriber::fmt::init();
    let cfg = Config::from_env();
    let responses_client = ResponsesClient::new(&cfg)?;
    let bot = Bot::new(cfg.teloxide_token.clone());

    let commands = vec![
        BotCommand::new("login", "Login to the service."),
        BotCommand::new("getwallet", "Get your wallet address."),
        BotCommand::new(
            "getbalance",
            "Get your balance for all tokens or a specific token (e.g. /getbalance or /getbalance SUI)",
        ),
        BotCommand::new("prompt", "Send a prompt to the AI."),
        BotCommand::new("promptexamples", "Show prompt examples."),
        BotCommand::new("help", "Display this help message."),
    ];
    bot.set_my_commands(commands).await?;

    Dispatcher::builder(bot, handler_tree())
        .dependencies(dptree::deps![
            responses_client.clone(),
            InMemStorage::<LoginState>::new(),
            squard_connect_client
        ])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}
