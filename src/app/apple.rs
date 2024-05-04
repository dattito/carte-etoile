use chrono::{DateTime, TimeZone, Utc};
use tracing::{info, warn};

use crate::{
    db::{queries::pass_registered_for_device, DbDevice, DbPass},
    Result,
};

use super::App;

impl App {
    pub async fn apple_device_registration(
        &self,
        device_library_id: &str,
        serial_number: &str,
        push_token: &str,
    ) -> Result<bool> {
        let already_exists =
            pass_registered_for_device(device_library_id, serial_number, &self.db_pool).await?;

        let now = chrono::Utc::now().naive_utc();
        let mut transaction = self.db_pool.begin().await?;

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
        .bind(push_token)
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
        .bind(serial_number)
        .bind(now)
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        info!(
            device_library_id = device_library_id,
            serial_number = serial_number,
            "new device registration"
        );

        Ok(already_exists)
    }

    pub async fn apple_device_deregistration(
        &self,
        device_library_id: &str,
        serial_number: &str,
    ) -> Result<()> {
        sqlx::query(
            "
DELETE FROM device_pass_registrations WHERE device_library_id=$1 AND pass_serial_number=$2
",
        )
        .bind(device_library_id)
        .bind(serial_number)
        .execute(&self.db_pool)
        .await?;

        if DbDevice::count_of_passes(device_library_id, &self.db_pool).await? == 0 {
            sqlx::query(
                "
DELETE FROM devices WHERE device_library_id=$1
        ",
            )
            .bind(device_library_id)
            .execute(&self.db_pool)
            .await?;

            info!("device deleted");
        }

        info!(devie_library_id=device_library_id, "device deregistered successfully");

        Ok(())
    }

    pub async fn apple_updatable_passes(
        &self,
        pass_type_id: &str,
        device_library_id: &str,
        passes_updated_since: Option<DateTime<Utc>>,
    ) -> Result<(Vec<String>, DateTime<Utc>)> {
        let passes = DbPass::from_pass_type_last_updated_device_library_id(
            pass_type_id,
            passes_updated_since.map(|d| d.naive_utc()),
            device_library_id,
            &self.db_pool,
        )
        .await?;

        let serial_numbers = passes
            .iter()
            .map(|pass| pass.serial_number.clone())
            .collect::<Vec<_>>();

        let last_updated = passes.iter().map(|pass| pass.last_updated_at).fold(
            chrono::Utc::now().naive_utc(),
            |acc, e| if acc < e { acc } else { e },
        );

        Ok((serial_numbers, Utc.from_utc_datetime(&last_updated)))
    }
}
