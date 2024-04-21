use tracing::info;

use crate::wallet_webhook::extractors::{Auth, RegistrationPath, RegistrationPushToken};

pub async fn handle_registration(
    Auth(token): Auth,
    RegistrationPath {
        device_id,
        pass_type_id,
        serial_number,
    }: RegistrationPath,
    RegistrationPushToken { push_token }: RegistrationPushToken,
) {
    info!(
        device_id = device_id,
        pass_type_id = pass_type_id,
        serial_number = serial_number,
        push_token = push_token,
        token = token,
        "new registration"
    );
}
