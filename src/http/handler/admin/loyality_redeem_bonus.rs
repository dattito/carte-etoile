use aide::transform::TransformOperation;
use axum::extract::{Path, State};

use crate::{http::AppState, Result};

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct LoyalityCardRedeemBonusPathParams {
    pub serial_number: String,
}

pub async fn handle_loyality_card_redeem_bonus(
    State(state): State<AppState>,
    Path(LoyalityCardRedeemBonusPathParams { serial_number }): Path<
        LoyalityCardRedeemBonusPathParams,
    >,
) -> Result<()> {
    state.app.pass_loyality_redeem_bonus(&serial_number).await
}

pub fn handle_loyality_card_redeem_bonus_docs(op: TransformOperation) -> TransformOperation {
    op.description("Redeem a full loyality card")
        .security_requirement("ApiKey")
        .tag("Shop Operator")
        .response::<200, ()>()
}
