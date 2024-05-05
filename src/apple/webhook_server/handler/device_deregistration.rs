use axum::{extract::State, http::StatusCode};

use crate::{
    apple::webhook_server::extractors::{DeviceLibraryId, SerialNumber},
    http::AppState,
    Result,
};

pub async fn handle_device_deregistration(
    State(state): State<AppState>,
    DeviceLibraryId { device_library_id }: DeviceLibraryId,
    SerialNumber(serial_number): SerialNumber,
) -> Result<StatusCode> {
    state
        .app
        .apple_device_unregistration(&device_library_id, &serial_number)
        .await?;

    Ok(StatusCode::OK)
}
