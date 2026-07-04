use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Redis connection URL
    pub redis_url: String,
    /// Local moka cache max capacity (number of entries)
    pub local_capacity: u64,
    /// Default TTL in seconds for cache entries if not specified per-request
    pub default_ttl_secs: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            redis_url: "redis://127.0.0.1/".to_owned(),
            local_capacity: 10_000,
            default_ttl_secs: 3600,
        }
    }
}
