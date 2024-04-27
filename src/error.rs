use axum::{
    extract::rejection::{JsonRejection, PathRejection},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::typed_header::TypedHeaderRejection;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("an unknown error occured")]
    Unknown,

    #[error("invalid requeest: {0}")]
    InvalidRequest(String),

    #[error("environment variable does not exist: {0}")]
    EnvVarDoesNotExist(String),

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
}

impl From<StatusCode> for Error {
    fn from(value: StatusCode) -> Self {
        Self::HttpStatus(value)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Unknown
            | Self::IO(_)
            | Self::OpenSsl(_)
            | Self::EnvVarDoesNotExist(_)
            | Self::DatabaseMigration(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong").into_response()
            }
            Self::AxumPathRejection(e) => e.into_response(),
            Self::AxumJsonRejection(e) => e.into_response(),
            Self::AxumTypedHeaderRejection(e) => e.into_response(),
            Self::InvalidRequest(message) => (StatusCode::BAD_REQUEST, message).into_response(),
            Self::Database(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong").into_response()
            }
            Self::HttpStatus(status_code) => status_code.into_response(),
        }
    }
}
