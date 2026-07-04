//! # microkernel-auth
//!
//! JWT 认证业务插件。

pub mod config;
pub mod port;

use std::sync::Arc;
use microkernel_contracts::{AppError, Plugin, SystemEnv};
use microkernel_macros::Plugin;

pub use port::AuthHandle;

#[derive(Plugin)]
pub struct AuthPlugin {
    config: config::AuthConfig,
    handle: Option<Arc<AuthHandle>>,
}

impl AuthPlugin {
    pub fn new(config: config::AuthConfig) -> Self {
        Self {
            config,
            handle: None,
        }
    }

    pub fn handle(&self) -> Option<Arc<AuthHandle>> {
        self.handle.clone()
    }
}

impl<E: SystemEnv> Plugin<E> for AuthPlugin {
    async fn on_load(&mut self, _env: &E) -> Result<(), AppError> {
        // 验证配置
        if self.config.secret.is_empty() {
            return Err(AppError::Config("JWT secret cannot be empty".to_owned()));
        }
        
        self.handle = Some(Arc::new(AuthHandle {
            secret: self.config.secret.clone(),
            expiration_secs: self.config.expiration_secs,
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
