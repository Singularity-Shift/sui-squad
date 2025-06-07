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

use dotenvy::dotenv;
use router::router;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let app = router().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3200").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
