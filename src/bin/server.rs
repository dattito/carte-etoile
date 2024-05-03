use std::sync::Arc;

use card_etoile::{
    app::App,
    apple::ApnClient,
    http::{self, InnerAppState},
    image::ImageMaker,
    utils::env::env_var,
    wallet::{ISignConfig, PassMaker},
    Result,
};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tracing::{info, Level};
use tracing_panic::panic_hook;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    std::panic::set_hook(Box::new(panic_hook));

    let config = Config::from_env()?;

    info!("Connecting to database...");

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    sqlx::migrate!().run(&db_pool).await.unwrap();

    let state = Arc::new(InnerAppState {
        app: App::new(
            PassMaker::new(
                ISignConfig::new(
                    &config.pass_signing_cert_path,
                    &config.pass_signing_key_path,
                    &config.pass_signing_key_token,
                )?,
                config.pass_team_identifier,
                config.pass_type_id,
                config.pass_web_service_url,
                config.pass_logo_path,
                config.pass_icon_path,
                ImageMaker::new(&config.background_image_path, &config.point_image_path)?,
            )?,
            db_pool.clone(),
        ),
        apn_client: ApnClient::new(
            &config.apn_signing_cert_p12_path,
            &config.apn_signing_cert_p12_token,
        )?,
        db_pool,
    });

    http::start(&config.http_listener_host, state).await?;

    Ok(())
}

struct Config {
    http_listener_host: String,
    database_url: String,
    pass_signing_cert_path: String,
    pass_signing_key_path: String,
    pass_signing_key_token: String,
    pass_team_identifier: String,
    pass_type_id: String,
    pass_web_service_url: String,
    pass_logo_path: String,
    pass_icon_path: String,
    apn_signing_cert_p12_path: String,
    apn_signing_cert_p12_token: String,
    background_image_path: String,
    point_image_path: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv().ok();

        Ok(Self {
            http_listener_host: env_var("HTTP_LISTENER_HOST").unwrap_or("127.0.0.1:3000".into()),
            database_url: env_var("DATABASE_URL")?,
            pass_signing_cert_path: env_var("PASS_SIGNING_CERT_PATH")?,
            pass_signing_key_path: env_var("PASS_SIGNING_KEY_PATH")?,
            pass_signing_key_token: env_var("PASS_SIGNING_KEY_TOKEN")?,
            pass_team_identifier: env_var("PASS_TEAM_IDENTIFIER")?,
            pass_type_id: env_var("PASS_TYPE_ID")?,
            pass_web_service_url: env_var("PASS_WEB_SERVICE_URL")?,
            pass_logo_path: env_var("PASS_LOGO_PATH")?,
            pass_icon_path: env_var("PASS_ICON_PATH")?,
            apn_signing_cert_p12_path: env_var("APN_SIGNING_CERT_P12_PATH")?,
            apn_signing_cert_p12_token: env_var("APN_SIGNING_CERT_P12_TOKEN")?,
            background_image_path: env_var("BACKGROUND_IMAGE_PATH")?,
            point_image_path: env_var("POINT_IMAGE_PATH")?,
        })
    }
}
