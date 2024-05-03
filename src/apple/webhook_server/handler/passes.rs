use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, HeaderName, StatusCode},
};

use crate::{
    apple::webhook_server::extractors::AuthToken,
    db::queries::correct_serial_number_auth_token,
    http::AppState,
    wallet::body_from_package,
    Result,
};

pub async fn handle_get_pass(
    State(state): State<AppState>,
    AuthToken(token): AuthToken,
    Path((pass_type_id, serial_number)): Path<(String, String)>,
) -> Result<([(HeaderName, &'static str); 2], Body)> {
    if !correct_serial_number_auth_token(&serial_number, &token, &state.db_pool).await? {
        return Err(StatusCode::UNAUTHORIZED.into());
    }

    let mut wallet_pass = state.app.pass_package(&pass_type_id, &serial_number).await?;

    let body = body_from_package(&mut wallet_pass)?;

    let headers = [
        (header::CONTENT_TYPE, "application/vnd.apple.pkpass"),
        (
            header::CONTENT_DISPOSITION,
            "attachment; filename=\"pass.pkpass\"",
        ),
    ];

    Ok((headers, body))
}
