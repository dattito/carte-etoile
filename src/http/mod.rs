use std::sync::Arc;

mod client_error;
mod docs;
mod handler;
mod middleware;
mod router;

pub use router::start;
use sqlx::PgPool;

use crate::{app::App, apple::ApnClient};

pub use client_error::ClientError;

pub use self::middleware::{OidcSub, OidcValidator};

pub type AppState = Arc<InnerAppState>;

#[derive(Debug)]
pub struct InnerAppState {
    pub app: App,
    pub db_pool: PgPool,
    pub apn_client: ApnClient,
    pub oidc_validator: OidcValidator,
}
