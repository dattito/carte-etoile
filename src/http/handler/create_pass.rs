use aide::transform::TransformOperation;
use axum::{
    body::Body,
    extract::State,
    http::{header, HeaderName},
};
use chrono::Utc;
use tracing::info;

use crate::{http::AppState, wallet::body_from_package, Result};

#[tracing::instrument(err, skip(state))]
pub async fn handle_create_pass(
    State(state): State<AppState>,
) -> Result<([(HeaderName, String); 3], Body)> {
    let (mut wallet_pass, serial_number) = state.app.add_pass("Test Name").await?;

    let body = body_from_package(&mut wallet_pass)?;

    let now = Utc::now().timestamp_millis().to_string();

    let headers = [
        (header::CONTENT_TYPE, "application/vnd.apple.pkpass".into()),
        (
            header::CONTENT_DISPOSITION,
            "attachment; filename=\"pass.pkpass\"".into(),
        ),
        ("last-modified".try_into().unwrap(), now),
    ];

    info!("created pass: {}", serial_number);

    Ok((headers, body))
}

pub fn handle_create_pass_docs(op: TransformOperation) -> TransformOperation {
    op.description("Create a new pass")
        .tag("Passes")
        .response_with::<200, (), _>(|res| res.description("Returns the new pass as a file"))
}
