mod admin;
mod docs;
mod error;
mod fund;
mod info;
mod middlewares;
mod payment;
mod router;
mod state;
mod user;
mod webhook;
mod withdraw;

use std::env;

use dotenvy::dotenv;
use router::router;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let server_domain = env::var("SERVER_DOMAIN").unwrap_or("localhost".to_string());

    let app = router().await;

    let listener = tokio::net::TcpListener::bind(&server_domain).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
