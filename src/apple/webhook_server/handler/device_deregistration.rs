use axum::{extract::State, http::StatusCode};

use crate::{
    apple::webhook_server::extractors::{DeviceLibraryId, PassAuth},
    http::AppState,
    Result,
};

#[tracing::instrument(err, skip(state))]
pub async fn handle_device_deregistration(
    State(state): State<AppState>,
    DeviceLibraryId { device_library_id }: DeviceLibraryId,
    PassAuth {
        serial_number,
        pass_type_id,
        pass_token: _,
    }: PassAuth,
) -> Result<StatusCode> {
    state
        .app
        .apple_device_deregistration(&device_library_id, &serial_number)
        .await?;

    Ok(StatusCode::OK)
}
