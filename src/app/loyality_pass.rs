use tracing::info;

use crate::{db::DbPassTypeLoyality, Error, Result};

use super::App;

impl App {
    pub async fn pass_loyality_add_points(
        &self,
        pass_serial_number: &str,
        points: i32,
    ) -> Result<()> {
        let pass =
            DbPassTypeLoyality::from_serial_number_optional(pass_serial_number, &self.db_pool)
                .await?
                .ok_or(Error::PassNotFound)?;

        if (pass.total_points - pass.current_points) < points || points == 0 {
            return Err(Error::InvalidAmountOfPoints);
        }

        DbPassTypeLoyality::add_points(pass_serial_number, points, &self.db_pool).await?;

        self.send_update_pass_notification(pass_serial_number)
            .await?;

        Ok(())
    }

    pub async fn pass_loyality_redeem_bonus(&self, pass_serial_number: &str) -> Result<()> {
        let pass =
            DbPassTypeLoyality::from_serial_number_optional(pass_serial_number, &self.db_pool)
                .await?
                .ok_or(Error::PassNotFound)?;

        if pass.total_points != pass.current_points {
            return Err(Error::InvalidAmountOfPoints);
        }

        DbPassTypeLoyality::redeem_bonus(pass_serial_number, &self.db_pool).await?;

        info!("Pass {pass_serial_number} successfully redeemed bonus");

        self.send_update_pass_notification(pass_serial_number)
            .await?;

        Ok(())
    }

    pub async fn get_loyality_pass(&self, pass_serial_number: &str) -> Result<DbPassTypeLoyality> {
        DbPassTypeLoyality::from_serial_number_optional(pass_serial_number, &self.db_pool)
            .await?
            .ok_or(Error::PassNotFound)
    }
}
