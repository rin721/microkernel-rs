use serde::{Deserialize, Serialize};

/// 关系数据库泛型应用配置。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbConfig {
    /// 数据库连接 URL（例如 `postgres://user:pass@localhost/db`）。
    pub url: String,
    /// 连接池中的最大并发连接数。
    pub max_connections: u32,
    /// 要维持的最小空闲连接数。
    pub min_connections: u32,
    /// 连接超时（秒）。
    pub connect_timeout: u64,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            url: "postgres://postgres:postgres@localhost:5432/postgres".to_owned(),
            max_connections: 10,
            min_connections: 1,
            connect_timeout: 5,
        }
    }
}
