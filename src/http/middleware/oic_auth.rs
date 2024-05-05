use std::{fmt, time::Duration};

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use oidc_jwt_validator::{
    cache::Strategy, FetchError, ValidationError, ValidationSettings, Validator,
};

use crate::http::AppState;

pub type OidcSub = String;

pub struct OidcValidator {
    issuer: String,
    validator: Validator,
}

impl fmt::Debug for OidcValidator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OidcValidator")
            .field("issuer", &self.issuer)
            .finish()
    }
}

#[derive(serde::Deserialize)]
pub struct TokenClaims {
    pub sub: String,
}

impl OidcValidator {
    pub async fn new(oidc_issuer: String) -> Result<Self, FetchError> {
        let client = reqwest::ClientBuilder::new()
            .timeout(Duration::from_secs(2))
            .build()
            .unwrap();

        let mut settings = ValidationSettings::new();
        settings.set_issuer(&[oidc_issuer.as_str()]);

        Ok(Self {
            issuer: oidc_issuer.clone(),
            validator: Validator::new(oidc_issuer, client, Strategy::Automatic, settings).await?,
        })
    }

    pub async fn validate(&self, token: &str) -> Result<TokenClaims, ValidationError> {
        let token = self.validator.validate::<TokenClaims>(token).await?;

        Ok(token.claims)
    }
}

pub async fn oidc_auth(
    State(state): State<AppState>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    mut req: Request,
    next: Next,
) -> crate::Result<Response> {
    let token = bearer.token();

    let valid_token = state.oidc_validator.validate(token).await?;

    req.extensions_mut().insert(valid_token.sub as OidcSub);

    Ok(next.run(req).await)
}
