mod env;
mod builder;

use microkernel_contracts::KernelError;
use builder::MicrokernelApp;

// 业务插件 (Plugins)
use microkernel_auth::{AuthPlugin, config::AuthConfig};
use microkernel_rbac::{RbacPlugin, config::RbacConfig};
use microkernel_mfa::{MfaPlugin, config::MfaConfig};
use microkernel_metrics::{MetricsPlugin, config::MetricsConfig};

#[tokio::main]
async fn main() -> Result<(), KernelError> {
    MicrokernelApp::builder()
        // 可选配置：通过 .with_logger(LoggerConfig::default()) 等按需修改
        .with_plugin(AuthPlugin::new(AuthConfig::default()))
        .with_plugin(RbacPlugin::new(RbacConfig::default()))
        .with_plugin(MfaPlugin::new(MfaConfig::default()))
        .with_plugin(MetricsPlugin::new(MetricsConfig::default()))
        .ignite()
        .await
}
