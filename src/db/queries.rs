use sqlx::PgPool;

pub async fn insert_device_if_not_exist(
    device_library_id: &str,
    device_push_token: &str,
    pass_type_id: &str,
    pass_serial_number: &str,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    let now = chrono::Utc::now().naive_utc();
    let mut transaction = pool.begin().await?;

    sqlx::query(
        "
INSERT INTO passes 
(serial_number, pass_type_id, created_at, last_updated_at) 
VALUES
($1, $2, $3, $4)
ON CONFLICT(serial_number) 
DO NOTHING",
    )
    .bind(pass_serial_number)
    .bind(pass_type_id)
    .bind(now)
    .bind(now)
    .execute(&mut *transaction)
    .await?;

    sqlx::query(
        "
INSERT INTO devices
(device_library_id, push_token, created_at, last_updated_at) 
VALUES
($1, $2, $3, $4)
ON CONFLICT (device_library_id) 
DO UPDATE SET push_token = $2, last_updated_at = $4
",
    )
    .bind(device_library_id)
    .bind(device_push_token)
    .bind(now)
    .bind(now)
    .execute(&mut *transaction)
    .await?;

    sqlx::query(
        "
INSERT INTO device_pass_registrations
(device_library_id, pass_serial_number, created_at)
VALUES
($1, $2, $3)
ON CONFLICT (device_library_id, pass_serial_number)
DO NOTHING
",
    )
    .bind(device_library_id)
    .bind(pass_serial_number)
    .bind(now)
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    Ok(())
}

pub async fn pass_registered_for_device(
    device_library_id: &str,
    pass_serial_number: &str,
    pool: &PgPool,
) -> Result<bool, sqlx::Error> {
    let result: (bool,) = sqlx::query_as(
        "
SELECT EXISTS (
    SELECT 1 
    FROM device_pass_registrations 
    WHERE pass_serial_number = 'serial'
    AND device_library_id = 'noice'
);
",
    )
    .bind(device_library_id)
    .bind(pass_serial_number)
    .fetch_one(pool)
    .await?;

    Ok(result.0)
}

pub async fn correct_serial_number_auth_token(
    serial_number: &str,
    auth_token: &str,
    pool: &PgPool,
) -> Result<bool, sqlx::Error> {
    let result: (bool,) = sqlx::query_as(
        "
SELECT EXISTS (
    SELECT 1 
    FROM passes 
    WHERE serial_number = $1
    AND auth_token = $2
);
",
    )
    .bind(serial_number)
    .bind(auth_token)
    .fetch_one(pool)
    .await?;

    Ok(result.0)
}
