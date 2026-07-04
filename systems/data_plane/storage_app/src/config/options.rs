use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StorageBackend {
    Local,
    S3,
    Oss,
}

impl Default for StorageBackend {
    fn default() -> Self {
        Self::Local
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub backend: StorageBackend,
    pub root: String,
    pub endpoint: Option<String>,
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
    pub bucket: Option<String>,
    pub region: Option<String>,
    pub extra: HashMap<String, String>,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            backend: StorageBackend::default(),
            root: "/tmp/microkernel_storage".to_owned(),
            endpoint: None,
            access_key: None,
            secret_key: None,
            bucket: None,
            region: None,
            extra: HashMap::new(),
        }
    }
}
