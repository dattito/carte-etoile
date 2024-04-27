use axum::{
    async_trait,
    extract::{FromRequest, Request},
    Json,
};

use crate::error::Error;

#[derive(serde::Deserialize)]
pub struct Logs {
    pub logs: Vec<String>,
}

#[async_trait]
impl<S> FromRequest<S> for Logs
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(r): Json<Self> = Json::from_request(req, state).await?;

        Ok(r)
    }
}
