use a2::{Client, DefaultNotificationBuilder, NotificationBuilder, NotificationOptions, Response};
use std::fs::File;

#[derive(Debug, Clone)]
pub struct ApnClient {
    base_client: Client,
}

impl ApnClient {
    pub fn new(certificate_path: &str, password: &str) -> Result<Self, a2::Error> {
        let mut certificate = File::open(certificate_path).unwrap();

        Ok(Self {
            base_client: Client::certificate(&mut certificate, password, a2::Endpoint::Production)?,
        })
    }

    pub async fn send_update_pass_notification(
        &self,
        device_token: &str,
    ) -> Result<Response, a2::Error> {
        let builder = DefaultNotificationBuilder::new();

        let options = NotificationOptions::default();

        let payload = builder.build(device_token, options);

        self.base_client.send(payload).await
    }
}
