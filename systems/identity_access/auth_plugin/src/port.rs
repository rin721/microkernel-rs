use microkernel_contracts::{AppError, AuthPort};
use microkernel_contracts::ports::Claims;
use std::collections::HashMap;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct JwtClaims {
    sub: String,
    exp: usize,
    #[serde(flatten)]
    extra: HashMap<String, String>,
}

pub struct AuthHandle {
    pub(crate) secret: String,
    pub(crate) expiration_secs: u64,
}

impl AuthPort for AuthHandle {
    async fn sign(&self, user_id: &str, extra: HashMap<String, String>) -> Result<String, AppError> {
        let claims = JwtClaims {
            sub: user_id.to_owned(),
            exp: (std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() + self.expiration_secs) as usize,
            extra,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        ).map_err(|e| AppError::PermissionDenied(format!("failed to generate token: {}", e)))
    }

    async fn verify(&self, token: &str) -> Result<Claims, AppError> {
        let token_data = decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        ).map_err(|e| AppError::PermissionDenied(format!("failed to verify token: {}", e)))?;

        Ok(Claims {
            subject: token_data.claims.sub,
            expires_at: token_data.claims.exp as i64,
            extra: token_data.claims.extra,
        })
    }
}
