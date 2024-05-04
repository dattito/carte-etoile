use tracing::info;

use crate::apple::webhook_server::extractors::Logs;

#[tracing::instrument]
pub async fn handle_log(body: String) {
    info!("device sent logs {}", body);
}
