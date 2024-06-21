use std::sync::Arc;

use carte_etoile::{
    app::{App, AppConfig},
    apple::ApnClient,
    db,
    http::{self, InnerAppState, OidcValidator},
    image::ImageMaker,
    setup_tracing,
    wallet::{ISignConfig, PassMaker},
    Result,
};
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    setup_tracing();

    let config = AppConfig::from_env()?;

    let db_pool = db::connect(&config.database_url).await?;

    let apn_client = ApnClient::new(
        &config.apn_signing_cert_p12_path,
        &config.apn_signing_cert_p12_token,
    )?;

    let sign_config = ISignConfig::new(
        &config.pass_signing_cert_path,
        &config.pass_signing_key_path,
        &config.pass_signing_key_token,
    )?;

    let pass_maker = PassMaker::new(
        sign_config,
        config.pass_team_identifier,
        config.pass_type_id,
        config.pass_web_service_url,
        config.pass_logo_path,
        config.pass_icon_path,
        ImageMaker::new(
            &config.background_image_path,
            &config.point_image_path,
            &config.bonus_point_image_path,
        )?,
    )?;

    let oidc_validator = OidcValidator::new(config.oidc_url).await?;

    let app = App::new(pass_maker, db_pool.clone(), apn_client.clone());

    let state = Arc::new(InnerAppState {
        app,
        apn_client,
        db_pool,
        oidc_validator,
    });

    http::start(&config.http_listener_host, state).await
}
