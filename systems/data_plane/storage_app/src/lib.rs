//! # microkernel-storage
//!
//! 统一对象存储泛型应用。
//! 包装 `opendal`（S3、本地文件系统、OSS 等）

pub mod config;
pub mod error;
pub mod lifecycle;
pub mod port;
pub mod provider;

use std::sync::Arc;

use config::StorageConfig;
use microkernel_contracts::{AppError, Archetype, HealthStatus, SystemEnv};

pub use port::StorageHandle;

pub struct StorageApp {
    config: StorageConfig,
    op: Option<opendal::Operator>,
    handle: Option<Arc<StorageHandle>>,
}

impl StorageApp {
    pub fn new(config: StorageConfig) -> Self {
        Self {
            config,
            op: None,
            handle: None,
        }
    }

    pub fn handle(&self) -> Option<Arc<StorageHandle>> {
        self.handle.clone()
    }
}

impl<E: SystemEnv> Archetype<E> for StorageApp {
    type Config = StorageConfig;

    fn default_config() -> Self::Config {
        StorageConfig::default()
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
        if self.handle.is_some() {
            Ok(HealthStatus::Healthy)
        } else {
            Ok(HealthStatus::Unhealthy)
        }
    }
}
