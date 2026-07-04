//! # microkernel-database
//!
//! 关系型数据库泛型应用。
//! 包装 `sea-orm`（其包装了 `sqlx`），以提供连接池管理
//! 和 schema 迁移。

pub mod config;
pub mod error;
pub mod lifecycle;
pub mod port;

use std::sync::Arc;

use config::DbConfig;
use microkernel_contracts::{AppError, Archetype, DatabasePort, HealthStatus, SystemEnv};

pub use port::DbHandle;

/// 数据库泛型应用。
pub struct DatabaseApp {
    config: DbConfig,
    /// 实际的数据库连接池，在 `pre_create` 期间建立。
    pool: Option<sea_orm::DatabaseConnection>,
    /// 暴露为端口的共享句柄。
    handle: Option<Arc<DbHandle>>,
}

impl DatabaseApp {
    /// 构造未配置的数据库应用。
    pub fn new(config: DbConfig) -> Self {
        Self {
            config,
            pool: None,
            handle: None,
        }
    }

    /// 返回数据库的共享句柄（在 `post_create` 后可用）。
    pub fn handle(&self) -> Option<Arc<DbHandle>> {
        self.handle.clone()
    }
}

impl<E: SystemEnv> Archetype<E> for DatabaseApp {
    type Config = DbConfig;

    fn default_config() -> Self::Config {
        DbConfig::default()
    }

    async fn pre_create(config: &mut Self::Config) -> Result<(), AppError> {
        lifecycle::pre_create::run(config).await
    }

    async fn post_create(&self) -> Result<(), AppError> {
        lifecycle::post_create::run(self).await
    }

    async fn pre_mount(&self, _env: &E) -> Result<(), AppError> {
        lifecycle::pre_mount::run(self).await
    }

    async fn post_mount(&self, _env: &E) -> Result<(), AppError> {
        lifecycle::post_mount::run(self).await
    }

    async fn pre_stop(&self) -> Result<(), AppError> {
        lifecycle::pre_stop::run(self).await
    }

    async fn post_stop(&self) -> Result<(), AppError> {
        lifecycle::post_stop::run(self).await
    }

    async fn health_check(&self) -> Result<HealthStatus, AppError> {
        if let Some(ref handle) = self.handle {
            match handle.ping().await {
                Ok(_) => Ok(HealthStatus::Healthy),
                Err(e) => Ok(HealthStatus::Degraded {
                    reason: format!("Ping failed: {}", e),
                }),
            }
        } else {
            Ok(HealthStatus::Unhealthy)
        }
    }
}
