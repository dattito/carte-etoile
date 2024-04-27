use axum::{middleware as axum_middleware, routing::get, Router};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tracing::info;

mod handler;
mod middleware;

use crate::{
    apple,
    http::{handler::handle_create_pass, middleware::setup_request_tracing},
    wallet::PassMaker,
    Error, Result,
};

#[derive(Clone, Debug)]
pub struct AppState {
    pub db_pool: PgPool,
    pub pass_maker: PassMaker,
}

pub async fn start(host: &str, db_pool: PgPool, pass_maker: PassMaker) -> Result<()> {
    let state = AppState {
        db_pool: db_pool.clone(),
        pass_maker,
    };

    let app = Router::new()
        .route("/health", get(|| async {}))
        .route(
            "/passes",
            get(handler::handle_create_pass).post(handle_create_pass),
        )
        .with_state(state.clone())
        .nest("/apple-webhooks", apple::router(state))
        .layer(axum_middleware::from_fn(setup_request_tracing));

    let listener = TcpListener::bind(host).await.unwrap();

    info!("Starting listening on {}", host);

    axum::serve(listener, app).await.map_err(Error::IO)
}
