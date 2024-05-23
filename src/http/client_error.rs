use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::Error;

pub enum ClientError {
    InternalServerError,
    NotFound,
    BadRequest(Option<String>),
    Unauthorized(Option<String>),
    JwtValidationCacheError,
    InvalidAccessToken,
    JwtMissingKidField,
    StatusCode(StatusCode),
    Extractor(Response),
}

impl IntoResponse for ClientError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Self::NotFound => StatusCode::NOT_FOUND.into_response(),
            Self::BadRequest(message) => ClientErrorResponse::new()
                .with_optional_message(message)
                .with_status_code(StatusCode::BAD_REQUEST)
                .into_response(),
            Self::Unauthorized(message) => ClientErrorResponse::new()
                .with_optional_message(message)
                .with_status_code(StatusCode::BAD_REQUEST)
                .into_response(),
            Self::JwtValidationCacheError => ClientErrorResponse::new()
                .with_message("error verifying jwt token")
                .with_status_code(StatusCode::INTERNAL_SERVER_ERROR)
                .into_response(),
            Self::InvalidAccessToken => ClientErrorResponse::new()
                .with_message("invalid access token")
                .with_status_code(StatusCode::UNAUTHORIZED)
                .into_response(),
            Self::JwtMissingKidField => ClientErrorResponse::new()
                .with_message(
                    "provided access token has no KID field in header, no chance to verify it",
                )
                .with_status_code(StatusCode::EXPECTATION_FAILED)
                .into_response(),
            Self::StatusCode(status_code) => status_code.into_response(),
            Self::Extractor(response) => response,
        }
    }
}

impl From<Error> for ClientError {
    fn from(value: Error) -> Self {
        match value {
            Error::Unknown
            | Error::IO(_)
            | Error::OpenSsl(_)
            | Error::Env(_)
            | Error::AppleApn(_)
            | Error::OidcValidateBuild(_)
            | Error::Image(_)
            | Error::Database(_)
            | Error::Other(_)
            | Error::DatabaseMigration(_) => Self::InternalServerError,
            Error::OidcValidate(e) => match e {
                oidc_jwt_validator::ValidationError::ValidationFailed(_)
                | oidc_jwt_validator::ValidationError::MissingKIDToken => Self::JwtMissingKidField,
                oidc_jwt_validator::ValidationError::CacheError => Self::JwtValidationCacheError,
                oidc_jwt_validator::ValidationError::MissingKIDJWKS => {
                    Self::JwtValidationCacheError
                }
            },
            Error::PassNotFound => Self::NotFound,
            Error::HttpStatus(sc) => Self::StatusCode(sc),
            Error::InvalidRequest(message) => Self::BadRequest(Some(message)),
            Error::InvalidAmountOfPoints => {
                Self::BadRequest(Some("invalid amount of points".into()))
            }
            Error::AxumPathRejection(rejection) => Self::Extractor(rejection.into_response()),
            Error::AxumJsonRejection(rejection) => Self::Extractor(rejection.into_response()),
            Error::AxumTypedHeaderRejection(rejection) => {
                Self::Extractor(rejection.into_response())
            }
        }
    }
}

#[derive(serde::Serialize)]
pub struct ClientErrorResponseMessage {
    pub message: String,
}

pub struct ClientErrorResponse {
    message: Option<String>,

    status_code: Option<StatusCode>,
}

impl ClientErrorResponse {
    pub fn new() -> Self {
        Self {
            message: None,
            status_code: None,
        }
    }

    pub fn with_message(mut self, message: impl ToString) -> Self {
        self.message = Some(message.to_string());
        self
    }

    pub fn with_optional_message(mut self, message: Option<impl ToString>) -> Self {
        self.message = message.map(|m| m.to_string());
        self
    }

    pub fn with_status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = Some(status_code);
        self
    }
}

impl IntoResponse for ClientErrorResponse {
    fn into_response(self) -> axum::response::Response {
        match (self.message, self.status_code) {
            (None, None) => ().into_response(),
            (Some(message), None) => Json(ClientErrorResponseMessage { message }).into_response(),
            (None, Some(status_code)) => status_code.into_response(),
            (Some(message), Some(status_code)) => {
                (status_code, Json(ClientErrorResponseMessage { message })).into_response()
            }
        }
    }
}
