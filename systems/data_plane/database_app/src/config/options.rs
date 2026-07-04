use serde::{Deserialize, Serialize};

/// Configuration for the relational database Generic App.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbConfig {
    /// The database connection URL (e.g., `postgres://user:pass@localhost/db`).
    pub url: String,
    /// Maximum number of concurrent connections in the pool.
    pub max_connections: u32,
    /// Minimum number of idle connections to maintain.
    pub min_connections: u32,
    /// Connection timeout in seconds.
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
