use axum::{
    http::header::{AUTHORIZATION, CONTENT_TYPE},
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{cors::Any, trace::TraceLayer};
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
            "/passes/{serial_number}/loyality/points",
            post(handler::handle_add_points_to_loyality_card),
        )
        .route(
            "/passes/{serial_number}/loyality/bonus",
            post(handler::handle_loyality_card_redeem_bonus),
        )
        .route(
            "/passes/{serial_number}/loyality",
            get(handler::handle_get_loyality_pass),
        )
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            oidc_auth,
        ))
        .route("/health", get(handler::handle_health))
        .route(
            "/passes",
            get(handler::handle_create_pass).post(handler::handle_create_pass),
        )
        .with_state(state.clone())
        .nest("/apple-webhooks", apple::router(state.clone()))
        .layer(
            ServiceBuilder::new()
                .layer(
                    tower_http::cors::CorsLayer::new()
                        .allow_methods(Any)
                        .allow_origin(Any)
                        .allow_headers([AUTHORIZATION, CONTENT_TYPE]),
                )
                .layer(axum::middleware::from_fn(setup_request_tracing))
                .layer(TraceLayer::new_for_http()),
        );

    let listener = TcpListener::bind(host).await.unwrap();

    info!("Starting listening on {}", host);

    axum::serve(listener, app.into_make_service())
        .await
        .map_err(Error::IO)
}
