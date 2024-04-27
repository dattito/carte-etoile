use axum::{extract::State, http::StatusCode};
use tracing::info;

use crate::{
    apple::extractors::{Auth, DeviceRegistrationPath},
    db::{queries::correct_serial_number_auth_token, DbDevice},
    http::AppState,
    Result,
};

#[tracing::instrument(err, skip(state, token))]
pub async fn handle_device_deregistration(
    State(state): State<AppState>,
    Auth(token): Auth,
    DeviceRegistrationPath {
        device_library_id,
        pass_type_id,
        serial_number,
    }: DeviceRegistrationPath,
) -> Result<StatusCode> {
    if !correct_serial_number_auth_token(&serial_number, &token, &state.db_pool).await? {
        info!("Called unauthorized");
        return Ok(StatusCode::UNAUTHORIZED);
    }

    sqlx::query(
        "
DELETE FROM device_pass_registrations WHERE device_library_id=$1 AND pass_serial_number=$2
",
    )
    .bind(&device_library_id)
    .bind(&serial_number)
    .execute(&state.db_pool)
    .await?;

    if DbDevice::count_of_passes(&device_library_id, &state.db_pool).await? == 0 {
        sqlx::query(
            "
DELETE FROM devices WHERE device_library_id=$1
        ",
        )
        .bind(&device_library_id)
        .execute(&state.db_pool)
        .await?;

        info!("device deleted");
    }

    info!("device deregistered successfully");

    Ok(StatusCode::OK)
}
