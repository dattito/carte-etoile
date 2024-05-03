use chrono::{TimeZone, Utc};
use passes::Package;

use crate::{
    db::{DbPass, DbPassTypeHelper, DbPassTypeLoyality},
    Result,
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
                _total_points: dbtl.total_points,
                _current_points: dbtl.current_points,
                pass_holder_name: dbtl.pass_holder_name,
                last_use: None,
            },
        )?;

        Ok((wallet_pass, serial_number))
    }

    pub async fn pass_package(&self, pass_type_id: &str, serial_number: &str) -> Result<Package> {
        let db_pass =
            DbPass::from_pass_type_serial_number(pass_type_id, serial_number, &self.db_pool)
                .await?;

        let pass_type = db_pass.r#type.query(serial_number, &self.db_pool).await?;

        let wallet_pass = match pass_type {
            crate::db::DbPassType::Loyality(l) => self.pass_maker.new_loyality_pass(
                serial_number.into(),
                db_pass.auth_token,
                crate::wallet::LoyalityPass {
                    already_redeemed: l.already_redeemed,
                    _total_points: l.total_points,
                    _current_points: l.current_points,
                    pass_holder_name: l.pass_holder_name,
                    last_use: l.last_used_at.map(|t| Utc.from_utc_datetime(&t)),
                },
            )?,
            // _ => return Err(Error::Other("not implemented".into())),
        };

        Ok(wallet_pass)
    }
}
