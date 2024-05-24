use aide::transform::TransformOperation;

pub async fn handle_log(body: String) {
    tracing::warn!(logs = body, "device sent logs");
}

pub fn handle_log_docs(op: TransformOperation) -> TransformOperation {
    op.description("Device Logs").response::<200, ()>().tag("Apple Webhooks")
}
