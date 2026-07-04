//! # microkernel-metrics
//!
//! Prometheus 指标业务插件。

pub mod config;
pub mod port;

use std::sync::Arc;
use microkernel_contracts::{AppError, Plugin, SystemEnv};
use microkernel_macros::Plugin;

pub use port::MetricsHandle;

#[derive(Plugin)]
pub struct MetricsPlugin {
    config: config::MetricsConfig,
    handle: Option<Arc<MetricsHandle>>,
}

impl MetricsPlugin {
    pub fn new(config: config::MetricsConfig) -> Self {
        Self {
            config,
            handle: None,
        }
    }

    pub fn handle(&self) -> Option<Arc<MetricsHandle>> {
        self.handle.clone()
    }
}

impl<E: SystemEnv> Plugin<E> for MetricsPlugin {
    async fn on_load(&mut self, _env: &E) -> Result<(), AppError> {
        self.handle = Some(Arc::new(MetricsHandle {}));
        Ok(())
    }

    async fn on_start(&mut self, _env: &E) -> Result<(), AppError> {
        let builder = metrics_exporter_prometheus::PrometheusBuilder::new();
        builder
            .with_http_listener(self.config.listen_addr)
            .install()
            .map_err(|e| AppError::Initialization(format!("failed to start metrics exporter: {}", e)))?;
            
        tracing::info!(addr = %self.config.listen_addr, "Prometheus metrics exporter started");
        Ok(())
    }

    fn name(&self) -> &'static str {
        Self::plugin_name()
    }
}
