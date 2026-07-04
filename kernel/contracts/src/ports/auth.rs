use std::collections::HashMap;

use crate::errors::AppError;

/// Decoded JWT claims, presented as a plain key-value map.
///
/// Using `HashMap<String, String>` avoids leaking `jsonwebtoken` or `paseto`
/// types into the contract boundary while still providing full access to all
/// custom claims.
#[derive(Debug, Clone)]
pub struct Claims {
    /// The subject (e.g., user ID) — corresponds to the JWT `sub` field.
    pub subject: String,
    /// Token expiry as a Unix timestamp.
    pub expires_at: i64,
    /// All additional custom claims decoded from the token payload.
    pub extra: HashMap<String, String>,
}

/// Port Trait for JWT-based authentication.
///
/// Covers token issuance and verification only. Session storage and user lookup
/// are responsibilities of higher-level business plugins.
pub trait AuthPort: Send + Sync + 'static {
    /// Issue a signed JWT for the given subject, embedding any extra claims.
    ///
    /// # Arguments
    /// * `subject`    — the unique identifier for the principal (e.g., user UUID)
    /// * `extra`      — additional claims to embed in the payload
    ///
    /// Returns the compact serialized token string.
    fn sign(
        &self,
        subject: &str,
        extra: HashMap<String, String>,
    ) -> impl std::future::Future<Output = Result<String, AppError>> + Send;

    /// Verify a token string and decode its claims.
    ///
    /// Returns `AppError::PermissionDenied` if the token is expired, malformed,
    /// or the signature does not match.
    fn verify(
        &self,
        token: &str,
    ) -> impl std::future::Future<Output = Result<Claims, AppError>> + Send;
}
