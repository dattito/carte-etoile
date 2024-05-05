use std::sync::Arc;

mod handler;
mod middleware;
mod router;

pub use router::start;
use sqlx::PgPool;

use crate::{app::App, apple::ApnClient};

pub use self::middleware::{OidcSub, OidcValidator};

pub type AppState = Arc<InnerAppState>;

#[derive(Debug)]
pub struct InnerAppState {
    pub app: App,
    pub db_pool: PgPool,
    pub apn_client: ApnClient,
    pub oidc_validator: OidcValidator,
}
