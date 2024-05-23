use axum::{
    extract::rejection::{JsonRejection, PathRejection},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::typed_header::TypedHeaderRejection;

use crate::http::ClientError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("an unknown error occured")]
    Unknown,

    #[error("other error: {0}")]
    Other(String),

    #[error("invalid request: {0}")]
    InvalidRequest(String),

    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),

    #[error("axum path rejection: {0}")]
    AxumPathRejection(#[from] PathRejection),

    #[error("axum json rejection: {0}")]
    AxumJsonRejection(#[from] JsonRejection),

    #[error("axum typed header rejection: {0}")]
    AxumTypedHeaderRejection(#[from] TypedHeaderRejection),

    #[error("database connection error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("database connection error: {0}")]
    DatabaseMigration(#[from] sqlx::migrate::MigrateError),

    #[error("openssl error: {0}")]
    OpenSsl(#[from] openssl::error::ErrorStack),

    #[error("status error: {0}")]
    HttpStatus(StatusCode),

    #[error("apple apn error: {0}")]
    AppleApn(#[from] a2::Error),

    #[error("image error: {0}")]
    Image(#[from] image::ImageError),

    #[error("pass not found")]
    PassNotFound,

    #[error("invalid amount of points")]
    InvalidAmountOfPoints,

    #[error("jwk error: {0}")]
    OidcValidate(#[from] oidc_jwt_validator::ValidationError),

    #[error("jwk build error: {0}")]
    OidcValidateBuild(#[from] oidc_jwt_validator::FetchError),

    #[error("env error: {0}")]
    Env(#[from] envy::Error),
}

impl From<StatusCode> for Error {
    fn from(value: StatusCode) -> Self {
        Self::HttpStatus(value)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        ClientError::from(self).into_response()
    }
}
