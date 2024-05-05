use axum::{
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::{
    apple,
    http::{
        handler,
        middleware::{oidc_auth, setup_request_tracing},
    },
    Error, Result,
};

use super::AppState;

pub async fn start(host: &str, state: AppState) -> Result<()> {
    let app = Router::new()
        .route(
            "/passes/:serial_number/loyality/points",
            post(handler::handle_add_points_to_loyality_card),
        )
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            oidc_auth,
        ))
        .route("/health", get(|| async {}))
        .route(
            "/passes",
            get(handler::handle_create_pass).post(handler::handle_create_pass),
        )
        .with_state(state.clone())
        .nest("/apple-webhooks", apple::router(state))
        .layer(
            ServiceBuilder::new()
                .layer(axum::middleware::from_fn(setup_request_tracing))
                .layer(TraceLayer::new_for_http()),
        );

    let listener = TcpListener::bind(host).await.unwrap();

    info!("Starting listening on {}", host);

    axum::serve(listener, app).await.map_err(Error::IO)
}
