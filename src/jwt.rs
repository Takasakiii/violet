use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Serialize, Deserialize)]
pub struct JwtClaims {
    username: String,
}

pub struct Jwt {
    secret: String,
}

impl Jwt {
    pub fn new(config: &Config) -> Self {
        Self {
            secret: config.jwt_secret.clone(),
        }
    }

    pub fn create_jwt(&self, username: String) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = JwtClaims { username };
        jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
    }

    pub fn verify_jwt(&self, token: &str) -> Result<JwtClaims, jsonwebtoken::errors::Error> {
        let claims = jsonwebtoken::decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )?;

        Ok(claims.claims)
    }
}
