use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaConfig {
    pub issuer: String,
}

impl Default for MfaConfig {
    fn default() -> Self {
        Self {
            issuer: "Microkernel App".to_owned(),
        }
    }
}
