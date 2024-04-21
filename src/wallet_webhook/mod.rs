use axum::{routing::post, Router};

mod extractors;
mod handler;

pub fn router() -> Router {
    Router::new()
        .route(
            "/v1/devices/:device_id/registrations/:pass_type_id/:serial_number",
            post(handler::handle_registration),
        )
        .route("/v1/log", post(handler::handle_log))
}
