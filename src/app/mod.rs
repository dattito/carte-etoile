use sqlx::PgPool;

use crate::wallet::PassMaker;
mod passes;
mod apple;


#[derive(Debug)]
pub struct App {
    pass_maker: PassMaker,
    db_pool: PgPool,
}

impl App {
    pub fn new(pass_maker: PassMaker, db_pool: PgPool) -> Self {
        Self {pass_maker, db_pool}
    } 
}

