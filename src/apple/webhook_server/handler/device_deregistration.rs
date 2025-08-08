use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use crate::{apple::webhook_server::extractors::AuthToken, http::AppState, Result};

#[derive(serde::Deserialize)]
pub struct PathParams {
    pub device_library_id: String,
    pub serial_number: String,
}

pub async fn handle_device_deregistration(
    State(state): State<AppState>,
    Path(PathParams {
        device_library_id,
        serial_number,
    }): Path<PathParams>,
    _: AuthToken,
) -> Result<StatusCode> {
    state
        .app
        .apple_device_unregistration(&device_library_id, &serial_number)
        .await?;

    Ok(StatusCode::OK)
}
