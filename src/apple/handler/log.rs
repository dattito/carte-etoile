use tracing::info;

use crate::apple::extractors::{Auth, Logs};

#[tracing::instrument]
pub async fn handle_log(Auth(token): Auth, Logs { logs }: Logs) {
    info!("device sent logs");
}
