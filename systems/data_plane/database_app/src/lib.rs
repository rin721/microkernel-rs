//! # microkernel-database
//!
//! Relational database Generic App.
//! Wraps `sea-orm` (which wraps `sqlx`) to provide connection pool management
//! and schema migrations.

pub mod config;
pub mod error;
pub mod lifecycle;
pub mod port;

use std::sync::Arc;

use config::DbConfig;
use microkernel_contracts::{AppError, Archetype, DatabasePort, HealthStatus, SystemEnv};

pub use port::DbHandle;

/// The database Generic App.
pub struct DatabaseApp {
    config: DbConfig,
    /// The actual database connection pool, established during `pre_create`.
    pool: Option<sea_orm::DatabaseConnection>,
    /// The shared handle exposed as the port.
    handle: Option<Arc<DbHandle>>,
}

impl DatabaseApp {
    /// Construct an unconfigured database app.
    pub fn new(config: DbConfig) -> Self {
        Self {
            config,
            pool: None,
            handle: None,
        }
    }

    /// Return a shared handle to the database (available after `post_create`).
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
