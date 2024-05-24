use aide::transform::TransformOperation;
use axum::{extract::State, http::StatusCode};

use crate::{
    apple::webhook_server::extractors::{AuthToken, DeviceLibraryId, SerialNumber},
    http::AppState,
    Result,
};

pub async fn handle_device_deregistration(
    State(state): State<AppState>,
    DeviceLibraryId { device_library_id }: DeviceLibraryId,
    _: AuthToken,
    SerialNumber(serial_number): SerialNumber,
) -> Result<StatusCode> {
    state
        .app
        .apple_device_unregistration(&device_library_id, &serial_number)
        .await?;

    Ok(StatusCode::OK)
}

pub fn handle_device_deregistration_docs(op: TransformOperation) -> TransformOperation {
    op.description("Deregister a device for a pass")
        .tag("Apple Webhooks")
        .response::<200, ()>()
}
