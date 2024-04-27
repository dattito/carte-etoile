use sqlx::prelude::FromRow;

#[derive(FromRow)]
pub struct DbDevicePassRegistration {
    pub device_library_id: String,
    pub pass_serial_number: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
}
