use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Redis 连接 URL
    pub redis_url: String,
    /// 本地 moka 缓存最大容量（条目数）
    pub local_capacity: u64,
    /// 如果未按请求指定，缓存条目的默认 TTL（秒）
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
