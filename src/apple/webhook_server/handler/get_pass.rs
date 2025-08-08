use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, HeaderMap, HeaderName},
};
use chrono::{TimeZone, Utc};

use crate::{
    apple::webhook_server::extractors::AuthToken, http::AppState, wallet::body_from_package, Result,
};

pub async fn handle_get_pass(
    State(state): State<AppState>,
    _: AuthToken,
    Path((_, serial_number)): Path<(String, String)>,
) -> Result<(HeaderMap, Body)> {
    let (mut wallet_pass, last_updated_at) = state.app.pass_package(&serial_number).await?;

    let last_updated_at_timestamp = Utc
        .from_utc_datetime(&last_updated_at)
        .timestamp_millis()
        .to_string();

    let body = body_from_package(&mut wallet_pass)?;

    let mut headers = HeaderMap::new();

    headers.insert(
        header::CONTENT_TYPE,
        "application/vnd.apple.pkpass".parse().unwrap(),
    );

    headers.insert(
        header::CONTENT_DISPOSITION,
        "attachment; filename=\"pass.pkpass\"".parse().unwrap(),
    );

    headers.insert(
        HeaderName::from_static("last-modified"),
        last_updated_at_timestamp.parse().unwrap(),
    );

    Ok((headers, body))
}
