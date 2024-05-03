use axum::{
    routing::{get, post},
    Router,
};

use crate::http::AppState;

use super::{handler, middleware::check_pass_auth};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route(
            "/v1/devices/:device_library_id/registrations/:pass_type_id/:serial_number",
            post(handler::handle_device_registration)
                .delete(handler::handle_device_deregistration)
                .get(handler::handle_device_registration),
        )
        .route(
            "/v1/passes/:pass_type_id/:serial_number",
            get(handler::handle_get_pass),
        )
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(),
            check_pass_auth,
        ))
        .route(
            "/v1/devices/:device_library_id/registrations/:pass_type_id",
            get(handler::handle_list_updatable_passes),
        )
        .route("/v1/log", post(handler::handle_log))
        .with_state(state)
}
