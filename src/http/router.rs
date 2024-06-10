use std::sync::Arc;

use aide::{
    axum::{
        routing::{get_with, post_with},
        ApiRouter,
    },
    openapi::{OpenApi, Tag},
    transform::TransformOpenApi,
};
use axum::{
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        StatusCode,
    },
    Extension, Json,
};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{cors::Any, trace::TraceLayer};
use tracing::info;
use uuid::{NoContext, Timestamp, Uuid};

use crate::{
    apple,
    http::{
        docs::docs_routes,
        handler,
        middleware::{oidc_auth, setup_request_tracing},
    },
    Error, Result,
};

use super::{AppState, ClientError};

pub async fn start(host: &str, state: AppState) -> Result<()> {
    let mut open_api = OpenApi::default();

    aide::gen::extract_schemas(true);
    let app = ApiRouter::new()
        .api_route(
            "/passes/:serial_number/loyality/points",
            post_with(
                handler::handle_add_points_to_loyality_card,
                handler::handle_add_points_to_loyality_card_docs,
            ),
        )
        .api_route(
            "/passes/:serial_number/loyality/bonus",
            post_with(
                handler::handle_loyality_card_redeem_bonus,
                handler::handle_loyality_card_redeem_bonus_docs,
            ),
        )
        .api_route(
            "/passes/:serial_number/loyality",
            get_with(
                handler::handle_get_loyality_pass,
                handler::handle_get_loyality_pass_docs,
            ),
        )
        // .layer(axum::middleware::from_fn_with_state(
        //     state.clone(),
        //     oidc_auth,
        // ))
        .api_route(
            "/health",
            get_with(handler::handle_health, handler::handle_health_docs),
        )
        .api_route(
            "/passes",
            get_with(
                handler::handle_create_pass,
                handler::handle_create_pass_docs,
            )
            .post_with(
                handler::handle_create_pass,
                handler::handle_create_pass_docs,
            ),
        )
        .with_state(state.clone())
        .nest_api_service("/apple-webhooks", apple::router(state.clone()))
        .layer(
            ServiceBuilder::new()
                .layer(
                    tower_http::cors::CorsLayer::new()
                        .allow_methods(Any)
                        .allow_origin(Any)
                        .allow_headers([AUTHORIZATION, CONTENT_TYPE]),
                )
                .layer(axum::middleware::from_fn(setup_request_tracing))
                .layer(TraceLayer::new_for_http()),
        )
        .nest_api_service("/docs", docs_routes(state))
        .finish_api_with(&mut open_api, api_docs)
        .layer(Extension(Arc::new(open_api)));

    aide::gen::extract_schemas(false);

    let listener = TcpListener::bind(host).await.unwrap();

    info!("Starting listening on {}", host);

    axum::serve(listener, app.into_make_service())
        .await
        .map_err(Error::IO)
}

fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("Carte Etoile")
        .version("v0.0.1")
        .tag(Tag {
            name: "Apple Webhooks".into(),
            description: Some("Used by apple products to communicate with our service. Do not manually send requests to these endpoints".into()),
            ..Default::default()
        })
        .tag(Tag {
            name: "Passes".into(),
            description: Some("Manage passes".into()),
            ..Default::default()
        })
        .tag(Tag {
            name: "Documentation UI".into(),
            description: Some("UI for the API documentation".into()),
            ..Default::default()
        })
        .tag(Tag {
            name: "Shop Operator".into(),
            description: Some("Endpoints important for shop operators".into()),
            ..Default::default()
        })
        .security_scheme(
            "ApiKey",
            aide::openapi::SecurityScheme::ApiKey {
                location: aide::openapi::ApiKeyLocation::Header,
                name: "Authorization".into(),
                description: Some("The access token.".into()),
                extensions: Default::default(),
            },
        )
        .default_response_with::<Json<ClientError>, _>(|res| {
            res.example(ClientError {
                error_name: "TheErrorName",
                error_details: Some("this tells you what went wrong".into()),
                client_message: Some("This is a message that the user can see"),
                request_id: Some(Uuid::new_v7(Timestamp::from_unix(
                    NoContext, 1497624119, 1234,
                ))),
                status: StatusCode::BAD_REQUEST,
            })
        })
}
