use std::sync::Arc;

use axum::{
    Router, middleware,
    routing::{get, post},
};
use squad_connect::{client::squad_connect::SquadConnect, service::dtos::Network};
use std::env;
use sui_sdk::SuiClientBuilder;
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};

use crate::{
    admin::handler::get_account,
    docs::{dto::ApiDoc, handler::api_docs},
    fund::handler::fund,
    info::handler::info,
    middlewares::handler::auth,
    payment::handler::payment,
    state::KeeperState,
    user::handler::create_user_if_not_exists,
    webhook::handler::webhook,
    withdraw::handler::withdraw,
};
use tower_http::trace::TraceLayer;

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
    }
    .expect("Failed to build client");

    let squad_connect_client = SquadConnect::new(node, client_id, network, api_key);

    let doc = ApiDoc::openapi();

    let (admin, path) = get_account();

    let state = Arc::new(KeeperState::from((squad_connect_client, admin, path)));

    let auth_routers = Router::new()
        .route("/user", post(create_user_if_not_exists))
        .route("/payment", post(payment))
        .route("/withdraw", post(withdraw))
        .route_layer(middleware::from_fn(auth));

    Router::new()
        .merge(Redoc::with_url("/redoc", doc))
        .merge(auth_routers)
        .route("/", get(info))
        .route("/docs", get(api_docs))
        .route("/webhook/{token}", get(webhook))
        .route("/fund", post(fund))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
