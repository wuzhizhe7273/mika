use anyhow::anyhow;
use axum::{
    async_trait,
    extract::{FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    RequestPartsExt,
};
use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{result::{AppError, AppResult}, config::CONFIG};

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub id: i64,
    pub exp: usize,
}

pub struct Keys {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AppError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::Unauthorized)?;
        tracing::debug!("bearer get");
        let key=DecodingKey::from_secret(CONFIG.jwt_key.as_bytes());
        let claims = jsonwebtoken::decode::<Claims>(
            bearer.token(),
            &key,
            &Validation::default(),
        )
        .map_err(|_| AppError::Unauthorized)?;
        Ok(claims.claims)
    }
}

impl Claims {
    pub async fn encode(&self) -> AppResult<String> {
        let key=EncodingKey::from_secret(CONFIG.jwt_key.as_bytes());
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            self,
            &key
        )
        .map_err(|e| anyhow!(e))?;
        Ok(token)
    }
}