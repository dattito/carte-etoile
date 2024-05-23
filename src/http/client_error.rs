use axum::{http::StatusCode, response::IntoResponse};
use schemars::JsonSchema;
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

use crate::Error;

#[derive(Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientError {
    /// The unique name of the error
    pub error_name: &'static str,

    #[serde(skip)]
    pub status: StatusCode,

    /// The unique id of the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<Uuid>,

    /// Optional Additional error details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<Value>,

    /// The unique id of the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_message: Option<&'static str>,
}

impl ClientError {
    pub fn new_internal_server_error() -> Self {
        Self {
            error_name: "InternalServerError",
            error_details: Some("Something went wrong on our side".into()),
            status: StatusCode::INTERNAL_SERVER_ERROR,
            request_id: None,
            client_message: Some("Something went wrong"),
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
            | Error::DatabaseMigration(_) => Self::new_internal_server_error(),
            Error::OidcValidate(e) => match e {
                oidc_jwt_validator::ValidationError::ValidationFailed(_)
                | oidc_jwt_validator::ValidationError::MissingKIDToken
                | oidc_jwt_validator::ValidationError::MissingKIDJWKS => Self {
                    error_name: "JwtMissingKidField",
                    error_details: Some(
                        "The access token provided does not have the required shape.".into(),
                    ),
                    status: StatusCode::EXPECTATION_FAILED,
                    request_id: None,
                    client_message: Some(
                        "The auth token is not valid. Please try to log out and in again.",
                    ),
                },
                oidc_jwt_validator::ValidationError::CacheError => {
                    Self::new_internal_server_error()
                }
            },
            Error::PassNotFound => Self {
                error_name: "PassNotFound",
                error_details: Some("this pass does not exist".into()),
                status: StatusCode::NOT_FOUND,
                request_id: None,
                client_message: Some("the pass you search for does not exist."),
            },
            Error::InvalidRequest(message) => Self {
                error_name: "InvalidRequest",
                error_details: Some(message.into()),
                status: StatusCode::BAD_REQUEST,
                request_id: None,
                client_message: None,
            },
            Error::InvalidAmountOfPoints => Self {
                error_name: "InvalidAmountOfPoints",
                error_details: Some("the amount of points entered are not valid".into()),
                status: StatusCode::BAD_REQUEST,
                request_id: None,
                client_message: Some("The amount of points entered are not valid. Are they maybe lower / higher than possible?"),
            },
            Error::AxumPathRejection(rejection) => {
                Self {
                    error_name: "PathRejection",
                    request_id: None,
                    status: rejection.status(),
                    client_message: None,
                    error_details: Some(rejection.body_text().into()),
                }
            },
            Error::AxumJsonRejection(rejection) =>{
                Self {
                    error_name: "JsonRejection",
                    request_id: None,
                    status: rejection.status(),
                    client_message: None,
                    error_details: Some(rejection.body_text().into()),
                }
            },
            Error::AxumTypedHeaderRejection(rejection) => {
                Self {
                    error_name: "HeaderRejection",
                    request_id: None,
                    status: StatusCode::BAD_REQUEST,
                    client_message: None,
                    error_details: Some(rejection.to_string().into()),
                }
            },
        }
    }
}

impl IntoResponse for ClientError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status;
        let mut res = axum::Json(self).into_response();
        *res.status_mut() = status;
        res
    }
}
