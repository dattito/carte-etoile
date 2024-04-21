use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tracing::info;

use crate::{
    error::{Error, Result},
    wallet_webhook,
};

pub async fn start(host: &str) -> Result<()> {
    let app = Router::new()
        .route("/health", get(health))
        .nest("", wallet_webhook::router());

    let listener = TcpListener::bind(host).await.unwrap();

    info!("Listeing on {}", host);

    axum::serve(listener, app).await.map_err(Error::IO)
}

async fn health() -> &'static str {
    "OK"
}
