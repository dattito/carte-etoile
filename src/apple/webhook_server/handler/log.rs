pub async fn handle_log(body: String) {
    tracing::warn!(logs = body, "device sent logs");
}
