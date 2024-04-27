use axum::{
    async_trait,
    extract::{FromRequest, FromRequestParts, Path, Request},
    http::request::Parts,
    Json,
};

use crate::error::Error;

pub struct DeviceRegistrationPath {
    pub device_library_id: String,
    pub pass_type_id: String,
    pub serial_number: String,
}

pub struct DeviceRegistrationPushToken {
    pub push_token: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Req {
    pub push_token: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for DeviceRegistrationPath
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Path((device_library_id, pass_type_id, serial_number)): Path<(String, String, String)> =
            Path::from_request_parts(parts, state).await?;

        Ok(Self {
            device_library_id,
            pass_type_id,
            serial_number,
        })
    }
}

#[async_trait]
impl<S> FromRequest<S> for DeviceRegistrationPushToken
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(r): Json<Req> = Json::from_request(req, state).await?;

        Ok(Self {
            push_token: r.push_token,
        })
    }
}
