use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use squard_connect::{client::squard_connect::SquardConnect, service::dtos::Network};
use sui_sdk::SuiClientBuilder;
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};
use std::env;

use crate::{
    db, docs::{dto::ApiDoc, handler::api_docs}, info::handler::info, keep::handler::keep, state::KeeperState, webhook::handler::webhook
};

pub async fn router() -> Router {
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
        Network::Mainnet => SuiClientBuilder::default().build_mainnet().await,
        Network::Testnet => SuiClientBuilder::default().build_testnet().await,
        _ => SuiClientBuilder::default().build_devnet().await,
    };

    let squard_connect_client = SquardConnect::new(node.unwrap(), client_id, network, api_key);

    let doc = ApiDoc::openapi();

    let db = db::init_tree();

    let state = Arc::new(KeeperState::from((db, squard_connect_client)));

    Router::new()
        .merge(Redoc::with_url("/redoc", doc))
        .route("/", get(info))
        .route("/docs", get(api_docs))
        .route("/webhook/{token}", get(webhook))
        .route("/keep", post(keep)).with_state(state)
}
