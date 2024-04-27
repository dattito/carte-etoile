use sqlx::{prelude::FromRow, PgPool};

#[derive(FromRow)]
pub struct DbDevice {
    pub device_library_id: String,
    pub push_token: String,
    pub created_at: chrono::NaiveDateTime,
    pub last_updated_at: chrono::NaiveDateTime,
}

impl DbDevice {
    pub async fn count_of_passes(
        devic_library_id: &str,
        pool: &PgPool,
    ) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar(
            "SELECT COUNT(*) FROM device_pass_registrations WHERE device_library_id = $1",
        )
        .bind(devic_library_id)
        .fetch_one(pool)
        .await
    }
}
