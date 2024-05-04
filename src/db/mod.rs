mod models;
pub mod queries;

pub use models::*;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing::info;

pub async fn connect(database_url: &str) -> Result<PgPool, sqlx::Error> {
    info!("Connecting to database...");

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    sqlx::migrate!().run(&db_pool).await.unwrap();

    Ok(db_pool)
}
