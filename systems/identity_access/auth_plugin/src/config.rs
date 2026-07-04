use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    #[serde(skip_serializing)]
    pub secret: String,
    pub expiration_secs: u64,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            secret: "change_me_in_production".to_owned(),
            expiration_secs: 3600,
        }
    }
}

impl Drop for AuthConfig {
    fn drop(&mut self) {
        self.secret.zeroize();
    }
}
