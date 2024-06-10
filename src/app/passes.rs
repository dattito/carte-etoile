use ::futures::future::join_all;
use chrono::{TimeZone, Utc};
use passes::Package;
use tracing::info;

use crate::{
    db::{
        queries::{push_tokens_from_serial_number, remove_devices_with_push_tokens},
        DbPass, DbPassTypeHelper, DbPassTypeLoyality,
    },
    Error, Result,
};

use super::App;

impl App {
    pub async fn add_pass(&self, pass_holder_name: &str) -> Result<(Package, String)> {
        let now = chrono::Utc::now();

        let serial_number = uuid::Uuid::now_v7().to_string();
        let auth_token = uuid::Uuid::now_v7().to_string();

        // TODO: Should be in same transaction!
        let pass = DbPass {
            serial_number: serial_number.clone(),
            pass_type_id: self.pass_maker.pass_type_identifier().to_string(),
            auth_token: auth_token.clone(),
            created_at: now.naive_utc(),
            last_updated_at: now.naive_utc(),
            r#type: DbPassTypeHelper::Loyality,
        };

        pass.insert(&self.db_pool).await?;

        let dbtl = DbPassTypeLoyality {
            serial_number: serial_number.clone(),
            total_points: 10,
            current_points: 0,
            already_redeemed: 0,
            pass_holder_name: pass_holder_name.to_string(),
            last_used_at: None,
        };

        dbtl.insert(&self.db_pool).await?;

        let wallet_pass = self.pass_maker.new_loyality_pass(
            serial_number.clone(),
            auth_token,
            crate::wallet::LoyalityPass {
                already_redeemed: dbtl.already_redeemed,
                total_points: dbtl.total_points,
                current_points: dbtl.current_points,
                pass_holder_name: dbtl.pass_holder_name,
                last_use: None,
            },
        )?;

        Ok((wallet_pass, serial_number))
    }

    pub async fn pass_package(&self, pass_serial_number: &str) -> Result<Package> {
        let db_pass = DbPass::from_serial_number_optional(pass_serial_number, &self.db_pool)
            .await?
            .ok_or(Error::PassNotFound)?;

        let pass_type = db_pass
            .r#type
            .from_serial_number(pass_serial_number, &self.db_pool)
            .await?;

        let wallet_pass = match pass_type {
            crate::db::DbPassType::Loyality(l) => self.pass_maker.new_loyality_pass(
                pass_serial_number.into(),
                db_pass.auth_token,
                crate::wallet::LoyalityPass {
                    already_redeemed: l.already_redeemed,
                    total_points: l.total_points,
                    current_points: l.current_points,
                    pass_holder_name: l.pass_holder_name,
                    last_use: l.last_used_at.map(|t| Utc.from_utc_datetime(&t)),
                },
            )?,
            // _ => return Err(Error::Other("not implemented".into())),
        };

        Ok(wallet_pass)
    }

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

    async fn send_update_pass_notification(&self, pass_serial_number: &str) -> Result<()> {
        let push_tokens = push_tokens_from_serial_number(pass_serial_number, &self.db_pool).await?;

        let notification_results = join_all(
            push_tokens
                .iter()
                .map(|push_token| self.apn_client.send_update_pass_notification(push_token)),
        )
        .await;

        // If the notification throws an error because the push tokens were invalid, then the
        // devices should be removed to prevent getting blocked from Apple for sending to much
        // invalid notifications.
        let (dirty_push_token_errors, other_errors): (Vec<_>, Vec<_>) = notification_results
            .into_iter()
            .enumerate()
            .filter_map(|(i, nr)| nr.err().map(|err| (i, err)))
            .partition(|(_, nr)| match nr {
                a2::Error::ResponseError(e) => match &e.error {
                    Some(res_err) => matches!(
                        res_err.reason,
                        a2::ErrorReason::Unregistered
                            | a2::ErrorReason::BadDeviceToken
                            | a2::ErrorReason::DeviceTokenNotForTopic
                    ),
                    _ => false,
                },
                _ => false,
            });

        other_errors
            .into_iter()
            .for_each(|(_, err)| tracing::error!("An APN error occured: {}", err));

        if dirty_push_token_errors.is_empty() {
            return Ok(());
        };

        tracing::error!(
            count = dirty_push_token_errors.len(),
            "some devices have invalid push_tokens and must get deleted"
        );

        let dirty_push_tokens = dirty_push_token_errors
            .into_iter()
            .map(|(i, _)| push_tokens[i].as_str())
            .collect::<Vec<_>>();

        remove_devices_with_push_tokens(dirty_push_tokens, &self.db_pool).await?;

        Ok(())
    }
}
