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

    pub async fn delete(device_library_id: &str, conn: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM devices WHERE device_library_id=$1")
            .bind(device_library_id)
            .execute(conn)
            .await?;

        Ok(())
    }

    pub async fn remove_pass(
        device_library_id: &str,
        pass_serial_number: &str,
        conn: &PgPool,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "DELETE FROM device_pass_registrations WHERE device_library_id=$1 AND pass_serial_number=$2",
        )
        .bind(device_library_id)
        .bind(pass_serial_number)
        .execute(conn)
        .await?;

        Ok(())
    }
}
