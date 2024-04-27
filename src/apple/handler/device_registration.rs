use axum::{extract::State, http::StatusCode};
use tracing::info;

use crate::{
    apple::extractors::{Auth, DeviceRegistrationPath, DeviceRegistrationPushToken},
    db::queries::{correct_serial_number_auth_token, pass_registered_for_device},
    http::AppState,
    Result,
};

#[tracing::instrument(err, skip(state, push_token, token))]
pub async fn handle_device_registration(
    State(state): State<AppState>,
    Auth(token): Auth,
    DeviceRegistrationPath {
        device_library_id,
        pass_type_id,
        serial_number,
    }: DeviceRegistrationPath,
    DeviceRegistrationPushToken { push_token }: DeviceRegistrationPushToken,
) -> Result<StatusCode> {
    if !correct_serial_number_auth_token(&serial_number, &token, &state.db_pool).await? {
        return Err(StatusCode::UNAUTHORIZED.into());
    }

    let already_exists =
        pass_registered_for_device(&device_library_id, &serial_number, &state.db_pool).await?;

    let now = chrono::Utc::now().naive_utc();
    let mut transaction = state.db_pool.begin().await?;

    sqlx::query(
        "
INSERT INTO devices
(device_library_id, push_token, created_at, last_updated_at) 
VALUES
($1, $2, $3, $4)
ON CONFLICT (device_library_id) 
DO UPDATE SET push_token = $2, last_updated_at = $4
",
    )
    .bind(&device_library_id)
    .bind(push_token)
    .bind(now)
    .bind(now)
    .execute(&mut *transaction)
    .await?;

    sqlx::query(
        "
INSERT INTO device_pass_registrations
(device_library_id, pass_serial_number, created_at)
VALUES
($1, $2, $3)
ON CONFLICT (device_library_id, pass_serial_number)
DO NOTHING
",
    )
    .bind(&device_library_id)
    .bind(&serial_number)
    .bind(now)
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    match already_exists {
        true => Ok(StatusCode::OK),
        false => {
            info!(
                device_library_id = device_library_id,
                pass_type_id = pass_type_id,
                serial_number = serial_number,
                "new registration"
            );

            Ok(StatusCode::CREATED)
        }
    }
}
