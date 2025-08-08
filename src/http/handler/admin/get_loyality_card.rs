use axum::{
    extract::{Path, State},
    Json,
};
use chrono::{DateTime, TimeZone, Utc};

use crate::{http::AppState, Result};

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoyalityPassResponse {
    pub serial_number: String,
    pub already_redeemed: i32,
    pub total_points: i32,
    pub current_points: i32,
    pub pass_holder_name: String,
    pub last_used_at: Option<DateTime<Utc>>,
}

#[derive(serde::Deserialize)]
pub struct GetLoyalityPassPathParams {
    pub serial_number: String,
}

pub async fn handle_get_loyality_pass(
    State(state): State<AppState>,
    Path(GetLoyalityPassPathParams { serial_number }): Path<GetLoyalityPassPathParams>,
) -> Result<Json<GetLoyalityPassResponse>> {
    let loyality_pass = state.app.get_loyality_pass(&serial_number).await?;

    Ok(Json(GetLoyalityPassResponse {
        serial_number: loyality_pass.serial_number,
        already_redeemed: loyality_pass.already_redeemed,
        total_points: loyality_pass.total_points,
        current_points: loyality_pass.current_points,
        pass_holder_name: loyality_pass.pass_holder_name,
        last_used_at: loyality_pass
            .last_used_at
            .map(|d| Utc.from_utc_datetime(&d)),
    }))
}
