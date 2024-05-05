use crate::{utils::env::env_var, Result};

pub struct AppConfig {
    pub http_listener_host: String,
    pub database_url: String,
    pub pass_signing_cert_path: String,
    pub pass_signing_key_path: String,
    pub pass_signing_key_token: String,
    pub pass_team_identifier: String,
    pub pass_type_id: String,
    pub pass_web_service_url: String,
    pub pass_logo_path: String,
    pub pass_icon_path: String,
    pub apn_signing_cert_p12_path: String,
    pub apn_signing_cert_p12_token: String,
    pub background_image_path: String,
    pub point_image_path: String,
    pub oidc_url: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
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
            oidc_url: env_var("OIDC_URL")?,
        })
    }
}
