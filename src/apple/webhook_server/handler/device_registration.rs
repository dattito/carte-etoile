use axum::{extract::State, http::StatusCode};

use crate::{
    apple::webhook_server::extractors::{DeviceLibraryId, DeviceRegistrationPushToken, PassAuth},
    http::AppState,
    Result,
};

#[tracing::instrument(err, skip(state, push_token))]
pub async fn handle_device_registration(
    State(state): State<AppState>,
    DeviceLibraryId { device_library_id }: DeviceLibraryId,
    PassAuth {
        serial_number,
        pass_type_id,
        pass_token: _,
    }: PassAuth,
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
