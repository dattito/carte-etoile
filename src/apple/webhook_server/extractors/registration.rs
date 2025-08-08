use axum::{
    extract::{FromRequest, Request},
    Json, RequestExt,
};

use crate::error::Error;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceRegistrationPushToken {
    pub push_token: String,
}

impl<S> FromRequest<S> for DeviceRegistrationPushToken
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(s): Json<DeviceRegistrationPushToken> = req.extract_with_state(state).await?;
        Ok(s)
    }
}
