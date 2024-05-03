use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tracing::info;

use crate::{apple, http::{handler, middleware::setup_request_tracing}, Error, Result};

use super::AppState;

pub async fn start(host: &str, state: AppState) -> Result<()> {

    let app = Router::new()
        .route("/health", get(|| async {}))
        .route(
            "/passes",
            get(handler::handle_create_pass).post(handler::handle_create_pass),
        )
        .with_state(state.clone())
        .nest("/apple-webhooks", apple::router(state))
        .layer(axum::middleware::from_fn(setup_request_tracing));

    let listener = TcpListener::bind(host).await.unwrap();

    info!("Starting listening on {}", host);

    axum::serve(listener, app).await.map_err(Error::IO)
}


