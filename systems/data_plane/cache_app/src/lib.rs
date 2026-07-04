//! # microkernel-cache
//!
//! Two-level cache Generic App.
//! L1: `moka` (local memory)
//! L2: `redis` (distributed)

pub mod config;
pub mod error;
pub mod lifecycle;
pub mod port;

use std::sync::Arc;

use config::CacheConfig;
use microkernel_contracts::{AppError, Archetype, CachePort, HealthStatus, SystemEnv};

pub use port::CacheHandle;

/// The cache Generic App.
pub struct CacheApp {
    config: CacheConfig,
    /// Redis connection client
    redis_client: Option<redis::Client>,
    /// Shared handle exposed as the port
    handle: Option<Arc<CacheHandle>>,
}

impl CacheApp {
    pub fn new(config: CacheConfig) -> Self {
        Self {
            config,
            redis_client: None,
            handle: None,
        }
    }

    pub fn handle(&self) -> Option<Arc<CacheHandle>> {
        self.handle.clone()
    }
}

impl<E: SystemEnv> Archetype<E> for CacheApp {
    type Config = CacheConfig;

    fn default_config() -> Self::Config {
        CacheConfig::default()
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
                    reason: format!("Redis ping failed: {}", e),
                }),
            }
        } else {
            Ok(HealthStatus::Unhealthy)
        }
    }
}
