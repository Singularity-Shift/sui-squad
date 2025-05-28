mod docs;
mod info;
mod keep;
mod router;
mod webhook;
mod state;
mod db;
mod error;
mod admin;
mod user;

use router::router;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let app = router().await;

    let listener = tokio::net::TcpListener::bind("localhost:3200")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
