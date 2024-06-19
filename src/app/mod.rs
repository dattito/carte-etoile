use sqlx::PgPool;

use crate::{apple::ApnClient, wallet::PassMaker};
mod apple;
mod config;
mod loyality_pass;
mod pass;

pub use config::AppConfig;

#[derive(Debug)]
pub struct App {
    pass_maker: PassMaker,
    db_pool: PgPool,
    apn_client: ApnClient,
}

impl App {
    pub fn new(pass_maker: PassMaker, db_pool: PgPool, apn_client: ApnClient) -> Self {
        Self {
            pass_maker,
            db_pool,
            apn_client,
        }
    }
}
