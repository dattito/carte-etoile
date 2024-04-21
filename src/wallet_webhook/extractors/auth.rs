use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, HeaderValue}};
use axum_extra::{
    headers::{authorization::Credentials, Authorization},
    TypedHeader,
};

use crate::error::Error;

pub struct Auth(pub String);

#[derive(Clone, PartialEq, Debug)]
/// Token holder for Bearer Authentication, most often seen with oauth
pub struct ApplePass(pub String);


#[async_trait]
impl<S> FromRequestParts<S> for Auth
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(b)): TypedHeader<Authorization<ApplePass>> =
            TypedHeader::from_request_parts(parts, state).await?;

        Ok(Self(b.token().into()))
    }
}

// Mirror "Bearer" struct
impl ApplePass {
    /// View the token part as a `&str`.
    pub fn token(&self) -> &str {
        self.0.as_str()["ApplePass ".len()..].trim_start()
    }
}

impl Credentials for ApplePass {
    const SCHEME: &'static str = "ApplePass";

    fn decode(value: &HeaderValue) -> Option<Self> {
        debug_assert!(
            value.as_bytes()[..Self::SCHEME.len()].eq_ignore_ascii_case(Self::SCHEME.as_bytes()),
            "HeaderValue to decode should start with \"ApplePass ..\", received = {:?}",
            value,
        );

        value.to_str().ok().map(|t| ApplePass(t.into()))
    }

    fn encode(&self) -> HeaderValue {
        unimplemented!()
    }
}
