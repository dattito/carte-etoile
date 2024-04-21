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

    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),

    #[error("axum path rejection: {0}")]
    AxumPathRejection(#[from] PathRejection),

    #[error("axum json rejection: {0}")]
    AxumJsonRejection(#[from] JsonRejection),

    #[error("axum typed header rejection: {0}")]
    AxumTypedHeaderRejection(#[from] TypedHeaderRejection),

    #[error("invalid requeest: {0}")]
    InvalidRequest(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Unknown | Self::IO(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong").into_response()
            }
            Self::AxumPathRejection(e) => e.into_response(),
            Self::AxumJsonRejection(e) => e.into_response(),
            Self::AxumTypedHeaderRejection(e) => e.into_response(),
            Self::InvalidRequest(message) => (StatusCode::BAD_REQUEST, message).into_response(),
        }
    }
}
