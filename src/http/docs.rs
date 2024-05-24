use std::sync::Arc;

use aide::{
    axum::{
        routing::{get, get_with},
        ApiRouter, IntoApiResponse,
    },
    openapi::OpenApi,
    redoc::Redoc,
    scalar::Scalar,
};
use axum::{response::IntoResponse, Extension, Json};

use super::AppState;

pub fn docs_routes(state: AppState) -> ApiRouter {
    aide::gen::infer_responses(true);

    let router = ApiRouter::new()
        .api_route_with(
            "/",
            get_with(
                Scalar::new("/docs/api.json")
                    .with_title("Carte Etoile")
                    .axum_handler(),
                |op| op.description("This documentation page."),
            ),
            |p| p.tag("Documentation UI"),
        )
        .api_route_with(
            "/redoc",
            get_with(
                Redoc::new("/docs/api.json")
                    .with_title("Carte Etoile")
                    .axum_handler(),
                |op| op.description("This is the documentation of carte etoile"),
            ),
            |p| p.tag("Documentation UI"),
        )
        .route("/api.json", get(serve_docs))
        .with_state(state);

    aide::gen::infer_responses(false);

    router
}

async fn serve_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    Json(api).into_response()
}
