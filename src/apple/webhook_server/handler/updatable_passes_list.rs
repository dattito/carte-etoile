use axum::extract::{Path, Query, State};
use axum::Json;
use chrono::{DateTime, Utc};
use schemars;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::formats::Flexible;
use serde_with::TimestampMilliSeconds;

use crate::http::AppState;
use crate::Result;

#[serde_with::serde_as]
#[derive(JsonSchema, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Res {
    serial_numbers: Vec<String>,
    #[serde_as(as = "TimestampMilliSeconds<String, Flexible>")]
    #[schemars(with = "DateTime<Utc>")]
    last_updated: DateTime<Utc>,
}

#[serde_with::serde_as]
#[derive(Deserialize)]
//#[schemars(rename = "AppleWebhookListUpdatablePassesQueryParams")]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    //#[schemars(with = "DateTime<Utc>")]
    #[serde_as(as = "Option<TimestampMilliSeconds<String, Flexible>>")]
    pub passes_updated_since: Option<DateTime<Utc>>,
}

#[derive(Deserialize, JsonSchema)]
#[schemars(rename = "AppleWebhookListUpdatablePassesPathParams")]
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

    Ok(Json(Res {
        serial_numbers,
        last_updated,
    }))
}

// pub fn handle_list_updatable_passes_docs(op: TransformOperation) -> TransformOperation {
//     op.description("List all updatable passes")
//         .response_with::<200, Json<Res>, _>(|res| {
//             res.example(Res {
//                 last_updated: DateTime::from_timestamp_millis(1716553998000).unwrap(),
//                 serial_numbers: vec![
//                     "b26a2ede-9a73-439f-b044-60ea35185c3e".into(),
//                     "ce69817f-d832-4747-b7a6-f6e4c8fb3279".into(),
//                 ],
//             })
//         })
//         .tag("Apple Webhooks")
// }
