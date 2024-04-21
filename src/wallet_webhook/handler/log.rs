use tracing::info;

use crate::wallet_webhook::extractors::{Logs, Auth};

pub async fn handle_log(Auth(token): Auth, Logs {logs}: Logs) {
    info!(token=token,"device sent logs: {:?}", logs);
}
