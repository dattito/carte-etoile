use std::io::{Cursor, Seek, SeekFrom};

use axum::{
    body::Body,
    extract::State,
    http::{header, HeaderName},
};
use tokio_util::io::ReaderStream;
use tracing::info;

use crate::{
    db::DbPass,
    http::AppState,
    {Error, Result},
};

#[tracing::instrument(err, skip(state))]
pub async fn handle_create_pass(
    State(state): State<AppState>,
) -> Result<([(HeaderName, &'static str); 2], Body)> {
    let now = chrono::Utc::now().naive_utc();

    let serial_number = uuid::Uuid::now_v7().to_string();
    let auth_token = uuid::Uuid::now_v7().to_string();

    let mut wallet_pass = state
        .pass_maker
        .new_pass(serial_number.clone(), auth_token.clone())?;

    let pass = DbPass {
        serial_number: serial_number.clone(),
        pass_type_id: state.pass_maker.pass_type_identifier().to_string(),
        auth_token,
        created_at: now,
        last_updated_at: now,
    };

    pass.insert(&state.db_pool).await?;

    let mut buffer = Cursor::new(Vec::new());

    wallet_pass.write(&mut buffer).map_err(|_| Error::Unknown)?;

    let _ = buffer.seek(SeekFrom::Start(0))?;

    let stream = ReaderStream::new(buffer);
    let body = Body::from_stream(stream);

    let headers = [
        (header::CONTENT_TYPE, "application/vnd.apple.pkpass"),
        (
            header::CONTENT_DISPOSITION,
            "attachment; filename=\"pass.pkpass\"",
        ),
    ];

    info!("created pass: {}", serial_number);

    Ok((headers, body))
}
