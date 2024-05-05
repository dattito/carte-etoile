use axum::{extract::State, http::StatusCode};

use crate::{
    apple::webhook_server::extractors::{
        DeviceLibraryId, DeviceRegistrationPushToken, SerialNumber,
    },
    http::AppState,
    Result,
};

pub async fn handle_device_registration(
    State(state): State<AppState>,
    DeviceLibraryId { device_library_id }: DeviceLibraryId,
    SerialNumber(serial_number): SerialNumber,
    DeviceRegistrationPushToken { push_token }: DeviceRegistrationPushToken,
) -> Result<StatusCode> {
    let already_exists = state
        .app
        .apple_device_registration(&device_library_id, &serial_number, &push_token)
        .await?;

    match already_exists {
        true => Ok(StatusCode::OK),
        false => Ok(StatusCode::CREATED),
    }
}
