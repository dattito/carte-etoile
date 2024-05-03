use axum::{
    async_trait, extract::{FromRequest, FromRequestParts, Path, Request}, http::request::Parts, Json, RequestExt, RequestPartsExt
};

use crate::error::Error;

use super::AuthToken;

#[derive(serde::Deserialize)]
pub struct DeviceLibraryId {
    pub device_library_id: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for DeviceLibraryId
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Path(s): Path<Self> = parts.extract_with_state(state).await?;
        Ok(s)
    }
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceRegistrationPushToken {
    pub push_token: String,
}

#[async_trait]
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

pub struct PassAuth {
    pub pass_type_id: String,
    pub serial_number: String,
    pub pass_token: String,
}

#[derive(serde::Deserialize)]
struct PassAuthPath {
    pub pass_type_id: String,
    pub serial_number: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for PassAuth
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Path(p): Path<PassAuthPath> = parts.extract_with_state(state).await?;

        let AuthToken(pass_token) = parts.extract().await?;

        Ok(Self {
            pass_type_id: p.pass_type_id,
            serial_number: p.serial_number,
            pass_token,
        })
    }
}
