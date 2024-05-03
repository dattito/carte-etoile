use axum::{extract::Request, middleware::Next, response::Response};
use tracing::info;

#[derive(Clone)]
pub struct RequestContext {
    uuid: uuid::Uuid,
}

impl RequestContext {
    pub fn new() -> Self {
        Self {
            uuid: uuid::Uuid::now_v7(),
        }
    }
}

#[tracing::instrument(skip(next,req),fields(request.uri = req.uri().to_string(), request.method = req.method().to_string()), name="request")]
pub async fn setup_request_tracing(mut req: Request, next: Next) -> Response {
    let request_context = RequestContext::new();
    info!(uuid = request_context.uuid.to_string(), "request");
    req.extensions_mut().insert(request_context.clone());

    let mut res = next.run(req).await;

    res.headers_mut().insert(
        "x-request-id",
        request_context.uuid.to_string().parse().unwrap(),
    );

    info!(status_code = res.status().to_string(), "response");

    res
}
