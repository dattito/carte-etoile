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
        sqlx::query!("INSERT INTO pass_type_loyality (serial_number, already_redeemed, total_points, current_points, pass_holder_name, last_used_at) VALUES ($1, $2, $3, $4, $5, $6)",
    self.serial_number.clone(),
    self.already_redeemed,
    self.total_points,
    self.current_points,
    &self.pass_holder_name,
    self.last_used_at,
            )
            .execute(conn).await
    }

    pub async fn from_serial_number(
        serial_number: &str,
        conn: &PgPool,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM pass_type_loyality WHERE serial_number=$1",
            serial_number
        )
        .fetch_one(conn)
        .await
    }

    pub async fn from_serial_number_optional(
        serial_number: &str,
        conn: &PgPool,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM pass_type_loyality WHERE serial_number=$1",
            serial_number,
        )
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
        sqlx::query!("UPDATE pass_type_loyality SET current_points=current_points+$1, last_used_at=$2 WHERE serial_number=$3", points, now, serial_number)
            .execute(&mut *transaction)
            .await?;

        sqlx::query!(
            "UPDATE passes SET last_updated_at=$1 WHERE serial_number=$2",
            now,
            serial_number
        )
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }

    pub async fn redeem_bonus(serial_number: &str, conn: &PgPool) -> Result<(), sqlx::Error> {
        let now = Utc::now().naive_utc();

        sqlx::query!("UPDATE pass_type_loyality SET current_points=0, already_redeemed=already_redeemed+1, last_used_at=$1 WHERE serial_number=$2", now, serial_number)
            .execute(conn).await?;

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
        Ok(sqlx::query_scalar!(
            "SELECT EXISTS ( SELECT 1 FROM passes WHERE serial_number=$1)",
            serial_number
        )
        .fetch_one(conn)
        .await?
        .unwrap_or(false))
    }

    pub async fn insert(&self, conn: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!(
            "INSERT INTO passes (serial_number, pass_type_id, auth_token, created_at, last_updated_at, type) VALUES ($1, $2, $3, $4, $5, $6)",
            &self.serial_number,
            &self.pass_type_id,
            &self.auth_token,
            self.created_at,
            self.last_updated_at,
            self.r#type.clone() as _
        )
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
        sqlx::query_as!(
            Self,
            "SELECT serial_number, auth_token, created_at, last_updated_at, pass_type_id, type as \"type: _\" FROM passes WHERE serial_number=$1",
            serial_number
        )
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
            sqlx::query_as!(
                Self,
            "SELECT serial_number, auth_token, p.created_at, last_updated_at, pass_type_id, type as \"type: _\" FROM passes p INNER JOIN device_pass_registrations dpr ON p.serial_number=dpr.pass_serial_number WHERE pass_type_id=$1 AND device_library_id=$2 AND last_updated_at>=$3", pass_type_id, device_library_id, lu
        )
        .fetch_all(conn)
        .await
        } else {
            sqlx::query_as!(
                Self,
            "SELECT serial_number, auth_token, p.created_at, last_updated_at, pass_type_id, type as \"type: _\" FROM passes p INNER JOIN device_pass_registrations dpr ON p.serial_number=dpr.pass_serial_number WHERE pass_type_id=$1 AND device_library_id=$2", pass_type_id, device_library_id,
        )
        .fetch_all(conn)
        .await
        }
    }

    pub async fn count_of_devices(serial_number: &str, conn: &PgPool) -> Result<i64, sqlx::Error> {
        Ok(sqlx::query_scalar!(
            "SELECT COUNT(*) FROM device_pass_registrations WHERE pass_serial_number = $1",
            serial_number
        )
        .fetch_one(conn)
        .await?
        .unwrap_or(0))
    }

    pub async fn delete(serial_number: &str, conn: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM passes WHERE serial_number=$1", serial_number)
            .execute(conn)
            .await?;

        Ok(())
    }
}
