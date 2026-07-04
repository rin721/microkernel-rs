use microkernel_contracts::AppError;
use totp_rs::{Algorithm, Secret, TOTP};

pub struct MfaHandle {
    pub(crate) issuer: String,
}

impl MfaHandle {
    pub async fn generate_secret(&self, account_name: &str) -> Result<(String, String), AppError> {
        let secret = Secret::Raw("12345678901234567890".as_bytes().to_vec());
        let totp = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            secret.to_bytes().unwrap(),
            Some(self.issuer.clone()),
            account_name.to_owned(),
        ).map_err(|e| AppError::PermissionDenied(e.to_string()))?;

        let qr = totp.get_url();
        Ok((secret.to_encoded().to_string(), qr))
    }

    pub async fn verify_token(&self, secret_base32: &str, token: &str) -> Result<bool, AppError> {
        let secret = Secret::Encoded(secret_base32.to_owned());
        let totp = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            secret.to_bytes().unwrap(),
            Some(self.issuer.clone()),
            "".to_owned(), // account name not needed for verification
        ).map_err(|e| AppError::PermissionDenied(e.to_string()))?;

        Ok(totp.check_current(token).unwrap_or(false))
    }
}
