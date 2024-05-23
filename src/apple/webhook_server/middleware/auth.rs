use axum::{
    extract::{Path, Request, State},
    middleware::Next,
    response::Response,
};

use crate::{
    apple::webhook_server::extractors::AuthToken, db::queries::correct_serial_number_auth_token,
    http::AppState, Error,
};

#[derive(serde::Deserialize)]
pub struct PathParams {
    pub serial_number: String,
}

pub async fn check_pass_auth(
    State(state): State<AppState>,
    Path(PathParams { serial_number }): Path<PathParams>,
    AuthToken(auth_token): AuthToken,
    req: Request,
    next: Next,
) -> Result<Response, Error> {
    if !correct_serial_number_auth_token(&serial_number, &auth_token, &state.db_pool).await? {
        return Err(Error::PassNotFound);
    }

    Ok(next.run(req).await)
}
