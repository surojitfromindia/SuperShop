use crate::config::load_env::EnvConfig;
use crate::types::PublicId;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize,Deserialize)]
pub struct Claims {
    pub user_id: String,
    pub exp: usize,
    pub iat: usize,
}
pub struct Token(String);

pub enum TokenType {
    AccessToken,
    RefreshToken,
}

#[derive(Debug, Error)]
pub enum TokenGenerationError{
    #[error("unknown token error")]
    UnknownError,
}

#[derive(Debug, Error)]
pub enum TokenVerificationError{
    #[error("token error {0}")]
    InvalidToken(String),
}

impl Token {
    // 1. func to generate token
    pub fn new(user_public_id : &PublicId ,token_type: &TokenType, env : &EnvConfig) -> anyhow::Result<Self,TokenGenerationError> {
        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let exp = match token_type {
            TokenType::AccessToken => (now + Duration::hours(1)).timestamp() as usize,
            TokenType::RefreshToken => (now + Duration::hours(24)).timestamp() as usize,
        };

        let claims = Claims {
            user_id : user_public_id.to_string(),
            exp,
            iat,
        };
        let token_string =
            encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(env.jwt_secret.as_bytes()),
            ).map_err(|_| TokenGenerationError::UnknownError)?;

        Ok(Token(token_string))
    }

    pub fn verify(token_string : &str , env : &EnvConfig) -> Result<Claims, TokenVerificationError > {
        let n =decode::<Claims>(
            token_string,
            &DecodingKey::from_secret(env.jwt_secret.as_bytes()),
            &Validation::default(),
        ).map_err(|e| TokenVerificationError::InvalidToken(e.to_string()))?;
        Ok(n.claims)
    }
}
impl Into<String> for Token{
    fn into(self) -> String {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn generate_token() {
        let env = EnvConfig {
            prime_org_id: 0,
            debug_mode: false,
            jwt_secret: "8y734274t234t2384t2374t2742t32".to_string(),
        };
        let user_public_id = PublicId::from("222222".to_string());
        assert_eq!(Token::new(&user_public_id, &TokenType::AccessToken, &env).is_ok(), true);
    }

    #[test]
    pub fn verify_valid_token() {
        let env = EnvConfig {
            prime_org_id: 0,
            debug_mode: false,
            jwt_secret: "8y734274t234t2384t2374t2742t32".to_string(),
        };
        let user_public_id = PublicId::from("222222".to_string());
        let token = Token::new(&user_public_id, &TokenType::AccessToken, &env)
            .expect("Token generation failure");
        let token_string : String= token.into();
        assert_eq!(Token::verify(token_string.as_str(), &env).is_ok(), true);
    }

    #[test]

    pub fn verify_invalid_token() {
        let env = EnvConfig {
            prime_org_id: 0,
            debug_mode: false,
            jwt_secret: "8y734274t234t2384t2374t2742t32".to_string(),
        };
        assert_eq!(Token::verify("98948394839439839", &env).is_ok(), false);
    }
}