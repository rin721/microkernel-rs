//! # microkernel-mfa
//!
//! 多因素认证业务插件。

pub mod config;
pub mod port;

use std::sync::Arc;
use microkernel_contracts::{AppError, Plugin, SystemEnv};
use microkernel_macros::Plugin;

pub use port::MfaHandle;

#[derive(Plugin)]
pub struct MfaPlugin {
    config: config::MfaConfig,
    handle: Option<Arc<MfaHandle>>,
}

impl MfaPlugin {
    pub fn new(config: config::MfaConfig) -> Self {
        Self {
            config,
            handle: None,
        }
    }

    pub fn handle(&self) -> Option<Arc<MfaHandle>> {
        self.handle.clone()
    }
}

impl<E: SystemEnv> Plugin<E> for MfaPlugin {
    async fn on_load(&mut self, _env: &E) -> Result<(), AppError> {
        self.handle = Some(Arc::new(MfaHandle {
            issuer: self.config.issuer.clone(),
        }));
        Ok(())
    }

    async fn on_start(&mut self, _env: &E) -> Result<(), AppError> {
        Ok(())
    }

    fn name(&self) -> &'static str {
        Self::plugin_name()
    }
}
