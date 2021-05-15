use std::collections::BTreeMap;

use chrono::Utc;
use hmac::{Hmac, NewMac, crypto_mac::InvalidKeyLength};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;

use crate::config;

type Hmac256 = Hmac<Sha256>;

pub enum TokenStats<T> {
    NotValid,
    Valid(T)
}

pub struct Tokens {
    secret: Hmac256
}

impl Tokens {
    pub fn new() -> Result<Self, InvalidKeyLength> {
        let config_token = &config::get_jwt_secret()[..];
        let arr_token = config_token.as_bytes();
        let secret = Hmac256::new_varkey(arr_token)?;
        let result = Self {
            secret
        };
        Ok(result)
    }

    pub fn generate_token(&self, id_user: u64) -> Result<String, jwt::Error> {
        let mut clains = BTreeMap::new();
        clains.insert("id", id_user);
        let now = Utc::now();
        let now_timestamp = now.timestamp();
        clains.insert("created", now_timestamp as u64);
        let token_str = clains.sign_with_key(&self.secret)?;
        Ok(token_str)
    }

    pub fn verify_token(&self, token: String) -> TokenStats<u64> {
        let token_resp: Result<BTreeMap<String, u64>, jwt::Error> = token.verify_with_key(&self.secret);
        match token_resp {
            Ok(clains) => TokenStats::Valid(clains["id"]),
            Err(_) => TokenStats::NotValid
        }
    }
}
