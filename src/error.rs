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

    #[error("other error: {0}")]
    Other(String),

    #[error("invalid request: {0}")]
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
            | Self::AppleApn(_)
            | Self::OidcValidateBuild(_)
            | Self::Image(_)
            | Self::DatabaseMigration(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong").into_response()
            }
            Self::AxumPathRejection(e) => e.into_response(),
            Self::AxumJsonRejection(e) => e.into_response(),
            Self::AxumTypedHeaderRejection(e) => e.into_response(),
            Self::InvalidRequest(message) => (StatusCode::BAD_REQUEST, message).into_response(),
            Self::Database(e) => {
                tracing::error!(sqlx_error = e.to_string(), "an database error occured");
                (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong").into_response()
            }
            Self::HttpStatus(status_code) => status_code.into_response(),
            Self::Other(e) => {
                tracing::error!("an error occured: '{e}'");
                (StatusCode::INTERNAL_SERVER_ERROR, e).into_response()
            }
            Self::PassNotFound => StatusCode::NOT_FOUND.into_response(),
            Self::InvalidAmountOfPoints => (
                StatusCode::NOT_FOUND,
                "the amount of points is invalidthe amount of points is invalid",
            )
                .into_response(),
            Self::OidcValidate(e) => match e {
                oidc_jwt_validator::ValidationError::ValidationFailed(_)
                | oidc_jwt_validator::ValidationError::MissingKIDToken => {
                    (StatusCode::UNAUTHORIZED, "invalid authorization token").into_response()
                }
                oidc_jwt_validator::ValidationError::CacheError => {
                    tracing::error!("couldn't verify jwt token due to cache error");
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
                oidc_jwt_validator::ValidationError::MissingKIDJWKS => {
                    tracing::error!("jwt token has no kid field");
                    (StatusCode::UNAUTHORIZED, "invalid authorization token").into_response()
                }
            },
        }
    }
}
