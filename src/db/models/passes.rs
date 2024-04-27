use sqlx::{prelude::FromRow, PgPool};

#[derive(FromRow)]
pub struct DbPass {
    pub pass_type_id: String,
    pub serial_number: String,
    pub auth_token: String,
    pub last_updated_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}

impl DbPass {
    pub async fn insert(&self, conn: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO passes (serial_number, pass_type_id, auth_token, created_at, last_updated_at) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(&self.serial_number)
        .bind(&self.pass_type_id)
        .bind(&self.auth_token)
        .bind(self.created_at)
        .bind(self.last_updated_at)
        .execute(conn).await?;

        Ok(())
    }

    pub async fn from_pass_type_serial_number(
        pass_type_id: String,
        serial_number: String,
        conn: &PgPool,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM passes WHERE pass_type_id=$1 AND serial_number=$2")
            .bind(pass_type_id)
            .bind(serial_number)
            .fetch_one(conn)
            .await
    }
}
