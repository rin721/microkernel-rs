//! # microkernel-rbac
//!
//! RBAC/ABAC 访问控制业务插件。

pub mod config;
pub mod port;

use std::sync::Arc;
use microkernel_contracts::{AppError, Plugin, SystemEnv};
use microkernel_macros::Plugin;

pub use port::RbacHandle;

#[derive(Plugin)]
pub struct RbacPlugin {
    config: config::RbacConfig,
    handle: Option<Arc<RbacHandle>>,
}

impl RbacPlugin {
    pub fn new(config: config::RbacConfig) -> Self {
        Self {
            config,
            handle: None,
        }
    }

    pub fn handle(&self) -> Option<Arc<RbacHandle>> {
        self.handle.clone()
    }
}

impl<E: SystemEnv> Plugin<E> for RbacPlugin {
    async fn on_load(&mut self, _env: &E) -> Result<(), AppError> {
        self.handle = Some(Arc::new(RbacHandle {
            model_path: self.config.model_path.clone(),
            policy_path: self.config.policy_path.clone(),
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
