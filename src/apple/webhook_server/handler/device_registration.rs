use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    apple::webhook_server::extractors::{AuthToken, DeviceRegistrationPushToken},
    http::AppState,
    Result,
};

#[derive(serde::Deserialize)]
pub struct PathParams {
    pub device_library_id: String,
    pub serial_number: String,
}

pub async fn handle_device_registration(
    State(state): State<AppState>,
    Path(PathParams {
        device_library_id,
        serial_number,
    }): Path<PathParams>,
    _: AuthToken,
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
