use axum::{
    routing::{get, post},
    Router,
};

use crate::http::AppState;

use super::handler;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/v1/log", post(handler::handle_log))
        .route(
            "/v1/devices/:device_id/registrations/:pass_type_id/:serial_number",
            post(handler::handle_device_registration).delete(handler::handle_device_deregistration),
        )
        .route(
            "/v1/passes/:pass_type_id/:serial_number",
            get(handler::handle_get_pass),
        )
        .with_state(state)
}
