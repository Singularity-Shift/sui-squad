use super::dtos::{Storage, UserId};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub telegram_id: UserId,
    pub exp: i64, // Expiration time
    pub iat: i64, // Issued at
}

pub struct JwtManager {
    secret: String,
}

impl JwtManager {
    pub fn new() -> Self {
        let secret = env::var("SECRET").expect("SECRET environment variable not found");
        JwtManager { secret }
    }

    pub fn generate_token(
        &self,
        telegram_id: UserId,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();
        let expiration = now + Duration::days(7);

        let claims = Claims {
            telegram_id,
            exp: expiration.timestamp(),
            iat: now.timestamp(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let token_data: TokenData<Claims> = decode(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }

    pub fn is_token_valid(&self, token: &str) -> bool {
        match self.validate_token(token) {
            Ok(claims) => {
                let now = Utc::now().timestamp();
                claims.exp > now
            }
            Err(_) => false,
        }
    }

    pub fn get_or_generate_token(
        &self,
        existing_token: Option<&str>,
        telegram_id: UserId,
    ) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(token) = existing_token {
            if self.is_token_valid(token) {
                return Ok(token.to_string());
            }
        }

        // Generate new token if none exists or current one is invalid
        self.generate_token(telegram_id)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    pub fn validate_and_update_storage(
        &self,
        mut storage: Storage,
        telegram_id: UserId,
    ) -> Result<Storage, Box<dyn std::error::Error>> {
        let existing_token = if storage.jwt.is_empty() {
            None
        } else {
            Some(storage.jwt.as_str())
        };

        let token = self.get_or_generate_token(existing_token, telegram_id)?;
        storage.jwt = token;

        Ok(storage)
    }
}
