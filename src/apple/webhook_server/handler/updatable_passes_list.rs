use axum::extract::{Path, Query, State};
use axum::Json;
use chrono::{DateTime, Utc};
use serde_with::formats::Flexible;
use serde_with::TimestampMilliSeconds;
use tracing::warn;

use crate::http::AppState;
use crate::Result;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Res {
    serial_numbers: Vec<String>,
    #[serde_as(as = "TimestampMilliSeconds<String, Flexible>")]
    last_updated: DateTime<Utc>,
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    #[serde_as(as = "Option<TimestampMilliSeconds<String, Flexible>>")]
    pub passes_updated_since: Option<DateTime<Utc>>,
}

#[derive(serde::Deserialize)]
pub struct PathParams {
    pub device_library_id: String,
    pub pass_type_id: String,
}

#[tracing::instrument(skip(state))]
pub async fn handle_list_updatable_passes(
    Query(QueryParams {
        passes_updated_since,
    }): Query<QueryParams>,
    Path(PathParams {
        device_library_id,
        pass_type_id,
    }): Path<PathParams>,
    State(state): State<AppState>,
) -> Result<Json<Res>> {
    let (serial_numbers, last_updated) = state
        .app
        .apple_updatable_passes(&pass_type_id, &device_library_id, passes_updated_since)
        .await?;

    warn!("serial_numbers: {serial_numbers:?}, last_updated:{}", last_updated.to_rfc3339());

    Ok(Json(Res {
        serial_numbers,
        last_updated,
    }))
}
