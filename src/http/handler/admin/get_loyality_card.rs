use aide::transform::TransformOperation;
use axum::{
    extract::{Path, State},
    Json,
};
use chrono::{DateTime, TimeZone, Utc};

use crate::{db::DbPassTypeLoyality, http::AppState, Error, Result};

#[derive(serde::Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetLoyalityPassResponse {
    pub serial_number: String,
    pub already_redeemed: i32,
    pub total_points: i32,
    pub current_points: i32,
    pub pass_holder_name: String,
    pub last_used_at: Option<DateTime<Utc>>,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct GetLoyalityPassPathParams {
    pub serial_number: String,
}


pub async fn handle_get_loyality_pass(
    State(state): State<AppState>,
    Path(GetLoyalityPassPathParams {serial_number}): Path<GetLoyalityPassPathParams>,
) -> Result<Json<GetLoyalityPassResponse>> {
    let a = DbPassTypeLoyality::from_serial_number_optional(&serial_number, &state.db_pool)
        .await?
        .ok_or(Error::PassNotFound)?;

    Ok(Json(GetLoyalityPassResponse {
        serial_number: a.serial_number,
        already_redeemed: a.already_redeemed,
        total_points: a.total_points,
        current_points: a.current_points,
        pass_holder_name: a.pass_holder_name,
        last_used_at: a.last_used_at.map(|d| Utc.from_utc_datetime(&d)),
    }))
}

pub fn handle_get_loyality_pass_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get a loyality pass")
        .tag("Shop Operator")
        .security_requirement("ApiKey")
        .response_with::<200, Json<GetLoyalityPassResponse>, _>(|res| {
            res.example(GetLoyalityPassResponse {
                serial_number: "9c5eb3c8-7c34-4eff-97d4-1edf04d06e81".into(),
                pass_holder_name: "John Sugar".into(),
                current_points: 4,
                total_points: 10,
                already_redeemed: 2,
                last_used_at: Some(
                    DateTime::parse_from_rfc3339("2020-12-09 16:09:53+00:00")
                        .unwrap()
                        .into(),
                ),
            })
        })
}
