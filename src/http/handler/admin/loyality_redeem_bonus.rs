use aide::transform::TransformOperation;
use axum::extract::{Path, State};

use crate::{http::AppState, Result};

pub async fn handle_loyality_card_redeem_bonus(
    State(state): State<AppState>,
    Path((serial_number,)): Path<(String,)>,
) -> Result<()> {
    state.app.pass_loyality_redeem_bonus(&serial_number).await
}

pub fn handle_loyality_card_redeem_bonus_docs(op: TransformOperation) -> TransformOperation {
    op.description("Redeem a full loyality card")
        .security_requirement("ApiKey")
        .tag("Shop Operator")
        .response::<200, ()>()
}

