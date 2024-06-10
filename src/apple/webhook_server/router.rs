use aide::axum::{
    routing::{get_with, post_with},
    ApiRouter,
};
use axum::routing::get;

use crate::http::AppState;

use super::{
    handler::{
        self, handle_device_deregistration_docs, handle_device_registration_docs, handle_log_docs,
    },
    middleware::check_pass_auth,
};

pub fn router(state: AppState) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/v1/devices/:device_library_id/registrations/:pass_type_id/:serial_number",
            post_with(
                handler::handle_device_registration,
                handler::handle_device_registration_docs,
            )
            .get_with(
                handler::handle_device_registration,
                handle_device_registration_docs,
            )
            .delete_with(
                handler::handle_device_deregistration,
                handle_device_deregistration_docs,
            ),
        )
        .api_route(
            "/v1/passes/:pass_type_id/:serial_number",
            get_with(handler::handle_get_pass, handler::handle_get_pass_docs),
        )
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(),
            check_pass_auth,
        ))
        .route(
            "/v1/devices/:device_library_id/registrations/:pass_type_id",
            get(
                handler::handle_list_updatable_passes,
                // handler::handle_list_updatable_passes_docs,
            ),
        )
        .api_route("/v1/log", post_with(handler::handle_log, handle_log_docs))
        .with_state(state)
}
