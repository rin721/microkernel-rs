//! # microkernel-logger
//!
//! Structured async logging Generic App.
//! Wraps `tracing` + `tracing-subscriber` + `tracing-appender`.

pub mod config;
pub mod lifecycle;
pub mod port;

use std::sync::Arc;

use config::LoggerConfig;
use microkernel_contracts::{AppError, Archetype, HealthStatus, SystemEnv};

pub use port::LoggerHandle;

/// The logging Generic App.
///
/// Must be the **first** component registered in `Bootstrap` so that all
/// subsequent lifecycle hooks emit structured logs.
pub struct LoggerApp {
    config: LoggerConfig,
    /// Guard that keeps the non-blocking rolling-file writer alive.
    /// Dropped in `post_stop` to flush and close the file.
    _guard: Option<tracing_appender::non_blocking::WorkerGuard>,
    handle: Option<Arc<LoggerHandle>>,
}

impl LoggerApp {
    /// Construct an unconfigured logger app. Call `Bootstrap::register_archetype`
    /// with the result and supply a `LoggerConfig`.
    pub fn new(config: LoggerConfig) -> Self {
        Self {
            config,
            _guard: None,
            handle: None,
        }
    }

    /// Return a shared handle to the logger (available after `post_create`).
    pub fn handle(&self) -> Option<Arc<LoggerHandle>> {
        self.handle.clone()
    }
}

impl<E: SystemEnv> Archetype<E> for LoggerApp {
    type Config = LoggerConfig;

    fn default_config() -> Self::Config {
        LoggerConfig::default()
    }

    async fn pre_create(config: &mut Self::Config) -> Result<(), AppError> {
        lifecycle::pre_create::run(config)
    }

    async fn post_create(&self) -> Result<(), AppError> {
        lifecycle::post_create::run(self)
    }

    async fn pre_mount(&self, _env: &E) -> Result<(), AppError> {
        lifecycle::pre_mount::run(self)
    }

    async fn post_mount(&self, _env: &E) -> Result<(), AppError> {
        lifecycle::post_mount::run(self)
    }

    async fn pre_stop(&self) -> Result<(), AppError> {
        lifecycle::pre_stop::run(self)
    }

    async fn post_stop(&self) -> Result<(), AppError> {
        lifecycle::post_stop::run(self)
    }

    async fn health_check(&self) -> Result<HealthStatus, AppError> {
        Ok(HealthStatus::Healthy)
    }
}
