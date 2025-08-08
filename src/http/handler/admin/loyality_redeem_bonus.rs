use axum::extract::{Path, State};

use crate::{http::AppState, Result};

#[derive(serde::Deserialize)]
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
