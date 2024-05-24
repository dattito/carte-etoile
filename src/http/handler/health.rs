use aide::transform::TransformOperation;

pub async fn handle_health() {}

pub fn handle_health_docs(op: TransformOperation) -> TransformOperation {
    op.description("Simple health check").response::<200, ()>()
}
