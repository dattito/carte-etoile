use tracing::info;

use crate::apple::webhook_server::extractors::{AuthToken, Logs};

#[tracing::instrument]
pub async fn handle_log(AuthToken(token): AuthToken, Logs { logs }: Logs) {
    info!("device sent logs {:?}", logs);
}
