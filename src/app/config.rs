fn default_http_listener_host() -> String {
    "127.0.0.1:3000".into()
}

#[derive(serde::Deserialize, Debug)]
pub struct AppConfig {
    #[serde(default = "default_http_listener_host")]
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
    pub fn from_env() -> Result<Self, envy::Error> {
        envy::from_env()
    }
}
