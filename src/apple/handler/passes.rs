use std::io::{Cursor, Seek, SeekFrom};

use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, HeaderName, StatusCode},
};
use tokio_util::io::ReaderStream;

use crate::{
    apple::extractors::Auth,
    db::{queries::correct_serial_number_auth_token, DbPass},
    http::AppState,
    Error, Result,
};

pub async fn handle_get_pass(
    State(state): State<AppState>,
    Auth(token): Auth,
    Path((pass_type_id, serial_number)): Path<(String, String)>,
) -> Result<([(HeaderName, &'static str); 2], Body)> {
    if !correct_serial_number_auth_token(&serial_number, &token, &state.db_pool).await? {
        return Err(StatusCode::UNAUTHORIZED.into());
    }

    let _db_pass =
        DbPass::from_pass_type_serial_number(pass_type_id, serial_number.clone(), &state.db_pool)
            .await?;

    let mut buffer = Cursor::new(Vec::new());

    let mut wallet_pass = state.pass_maker.new_pass(serial_number, token.clone())?;

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

    Ok((headers, body))
}
