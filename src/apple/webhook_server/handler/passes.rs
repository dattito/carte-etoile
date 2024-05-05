use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, HeaderName},
};

use crate::{http::AppState, wallet::body_from_package, Result};

pub async fn handle_get_pass(
    State(state): State<AppState>,
    Path((_, serial_number)): Path<(String, String)>,
) -> Result<([(HeaderName, &'static str); 2], Body)> {
    let mut wallet_pass = state.app.pass_package(&serial_number).await?;

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
