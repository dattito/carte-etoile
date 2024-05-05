use chrono::{NaiveDateTime, Utc};
use sqlx::{postgres::PgQueryResult, prelude::FromRow, PgPool};

#[derive(FromRow)]
pub struct DbPassTypeLoyality {
    pub serial_number: String,
    pub already_redeemed: i32,
    pub total_points: i32,
    pub current_points: i32,
    pub pass_holder_name: String,
    pub last_used_at: Option<NaiveDateTime>,
}

impl DbPassTypeLoyality {
    pub async fn insert(&self, conn: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query("INSERT INTO pass_type_loyality (serial_number, already_redeemed, total_points, current_points, pass_holder_name, last_used_at) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(self.serial_number.clone())
            .bind(self.already_redeemed)
            .bind(self.total_points)
            .bind(self.current_points)
            .bind(&self.pass_holder_name)
            .bind(self.last_used_at)
            .execute(conn).await
    }

    pub async fn from_serial_number(
        serial_number: &str,
        conn: &PgPool,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as("SELECT * FROM pass_type_loyality WHERE serial_number=$1")
            .bind(serial_number)
            .fetch_one(conn)
            .await
    }

    pub async fn from_serial_number_optional(
        serial_number: &str,
        conn: &PgPool,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM pass_type_loyality WHERE serial_number=$1")
            .bind(serial_number)
            .fetch_optional(conn)
            .await
    }

    pub async fn add_points(
        serial_number: &str,
        points: i32,
        conn: &PgPool,
    ) -> Result<(), sqlx::Error> {
        let now = Utc::now().naive_utc();

        let mut transaction = conn.begin().await?;
        sqlx::query("UPDATE pass_type_loyality SET current_points=current_points+$1, last_used_at=$2 WHERE serial_number=$3")
        .bind(points)
            .bind(now).bind(serial_number).execute(&mut *transaction).await?;

        sqlx::query("UPDATE passes SET last_updated_at=$1 WHERE serial_number=$2")
            .bind(now)
            .bind(serial_number)
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;

        Ok(())
    }
}

pub enum DbPassType {
    Loyality(DbPassTypeLoyality),
}

impl DbPassType {
    pub async fn insert<T>(&self, conn: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        match self {
            Self::Loyality(l) => l.insert(conn).await,
        }
    }
}

#[derive(sqlx::Type, Clone, Debug)]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE", type_name = "pass_type")]
pub enum DbPassTypeHelper {
    Loyality,
}

impl DbPassTypeHelper {
    pub async fn from_serial_number(
        &self,
        serial_number: &str,
        conn: &PgPool,
    ) -> Result<DbPassType, sqlx::Error> {
        match self {
            Self::Loyality => Ok(DbPassType::Loyality(
                DbPassTypeLoyality::from_serial_number(serial_number, conn).await?,
            )),
        }
    }
}

impl From<DbPassType> for DbPassTypeHelper {
    fn from(value: DbPassType) -> Self {
        match value {
            DbPassType::Loyality(_) => Self::Loyality,
        }
    }
}

#[derive(FromRow, Debug)]
pub struct DbPass {
    pub serial_number: String,
    pub pass_type_id: String,
    pub auth_token: String,
    pub last_updated_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
    pub r#type: DbPassTypeHelper,
}

impl DbPass {
    pub async fn exists(serial_number: &str, conn: &PgPool) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar("SELECT EXISTS ( SELECT 1 FROM passes WHERE serial_number=$1)")
            .bind(serial_number)
            .fetch_one(conn)
            .await
    }

    pub async fn insert(&self, conn: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query(
            "INSERT INTO passes (serial_number, pass_type_id, auth_token, created_at, last_updated_at, type) VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(&self.serial_number)
        .bind(&self.pass_type_id)
        .bind(&self.auth_token)
        .bind(self.created_at)
        .bind(self.last_updated_at)
        .bind(self.r#type.clone())
        .execute(conn).await
    }

    pub async fn from_serial_number(
        serial_number: &str,
        conn: &PgPool,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as("SELECT * FROM passes WHERE serial_number=$1")
            .bind(serial_number)
            .fetch_one(conn)
            .await
    }

    pub async fn from_serial_number_optional(
        serial_number: &str,
        conn: &PgPool,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, _>("SELECT * FROM passes WHERE serial_number=$1")
            .bind(serial_number)
            .fetch_optional(conn)
            .await
    }

    pub async fn from_pass_type_last_updated_device_library_id(
        pass_type_id: &str,
        last_updated_at: Option<chrono::NaiveDateTime>,
        device_library_id: &str,
        conn: &PgPool,
    ) -> Result<Vec<Self>, sqlx::Error> {
        if let Some(lu) = last_updated_at {
            sqlx::query_as::<_, Self>(
            "SELECT * FROM passes p INNER JOIN device_pass_registrations dpr ON p.serial_number=dpr.pass_serial_number WHERE pass_type_id=$1 AND device_library_id=$2 AND last_updated_at>=$3",
        )
        .bind(pass_type_id)
        .bind(device_library_id)
        .bind(lu)
        .fetch_all(conn)
        .await
        } else {
            sqlx::query_as::<_, Self>(
            "SELECT * FROM passes p INNER JOIN device_pass_registrations dpr ON p.serial_number=dpr.pass_serial_number WHERE pass_type_id=$1 AND device_library_id=$2",
        )
        .bind(pass_type_id)
        .bind(device_library_id)
        .fetch_all(conn)
        .await
        }
    }
}
