//! # microkernel-logger
//!
//! 结构化异步日志记录通用应用。
//! 封装 `tracing` + `tracing-subscriber` + `tracing-appender`。

pub mod config;
pub mod lifecycle;
pub mod port;

use std::sync::Arc;

use config::LoggerConfig;
use microkernel_contracts::{AppError, Archetype, HealthStatus, SystemEnv};

pub use port::LoggerHandle;

/// 日志记录通用应用。
///
/// 必须是在 `Bootstrap` 中注册的**第一个**组件，以便所有
/// 后续生命周期钩子发出结构化日志。
pub struct LoggerApp {
    config: LoggerConfig,
    /// 保持非阻塞滚动文件写入器存活的守卫。
    /// 在 `post_stop` 中被丢弃以刷新并关闭文件。
    _guard: Option<tracing_appender::non_blocking::WorkerGuard>,
    handle: Option<Arc<LoggerHandle>>,
}

impl LoggerApp {
    /// 构造一个未配置的日志应用。用结果调用 `Bootstrap::register_archetype`
    /// 并提供一个 `LoggerConfig`。
    pub fn new(config: LoggerConfig) -> Self {
        Self {
            config,
            _guard: None,
            handle: None,
        }
    }

    /// 返回一个日志记录器的共享句柄（在 `post_create` 后可用）。
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
