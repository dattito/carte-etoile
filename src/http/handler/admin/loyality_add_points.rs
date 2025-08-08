use axum::extract::{Path, State};
use axum::Json;

use crate::{http::AppState, Result};

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonBody {
    add_points: u16,
}

#[derive(serde::Deserialize)]
pub struct AddPointsToLoyalityCardPathParams {
    pub serial_number: String,
}

pub async fn handle_add_points_to_loyality_card(
    State(state): State<AppState>,
    Path(AddPointsToLoyalityCardPathParams { serial_number }): Path<
        AddPointsToLoyalityCardPathParams,
    >,
    Json(JsonBody { add_points }): Json<JsonBody>,
) -> Result<()> {
    state
        .app
        .pass_loyality_add_points(&serial_number, add_points.into())
        .await
}
