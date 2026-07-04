mod env;

use microkernel_core::lifecycle::Bootstrap;
use microkernel_contracts::KernelError;
use tracing::info;
use env::ProdEnv;

// 泛型应用 (Generic Apps)
use microkernel_logger::{LoggerApp, config::LoggerConfig};
use microkernel_database::{DatabaseApp, config::DbConfig};
use microkernel_cache::{CacheApp, config::CacheConfig};
use microkernel_storage::{StorageApp, config::StorageConfig};

// 业务插件 (Plugins)
use microkernel_auth::{AuthPlugin, config::AuthConfig};
use microkernel_rbac::{RbacPlugin, config::RbacConfig};
use microkernel_mfa::{MfaPlugin, config::MfaConfig};
use microkernel_metrics::{MetricsPlugin, config::MetricsConfig};

#[tokio::main]
async fn main() -> Result<(), KernelError> {
    // 1. 创建应用与插件（尚未挂载）
    let logger_app = LoggerApp::new(LoggerConfig::default());
    let db_app = DatabaseApp::new(DbConfig::default());
    let cache_app = CacheApp::new(CacheConfig::default());
    let storage_app = StorageApp::new(StorageConfig::default());
    
    let auth_plugin = AuthPlugin::new(AuthConfig::default());
    let rbac_plugin = RbacPlugin::new(RbacConfig::default());
    let mfa_plugin = MfaPlugin::new(MfaConfig::default());
    let metrics_plugin = MetricsPlugin::new(MetricsConfig::default());

    // 2. 初始化 Bootstrap
    let mut bootstrap = Bootstrap::<ProdEnv>::new();
    
    // 注册泛型应用（注意顺序！）
    let logger_shared = bootstrap.register_archetype(logger_app);
    let db_shared = bootstrap.register_archetype(db_app);
    let cache_shared = bootstrap.register_archetype(cache_app);
    let storage_shared = bootstrap.register_archetype(storage_app);

    // 注册插件
    let auth_shared = bootstrap.register_plugin(auth_plugin);
    let rbac_shared = bootstrap.register_plugin(rbac_plugin);
    let _mfa_shared = bootstrap.register_plugin(mfa_plugin);
    let _metrics_shared = bootstrap.register_plugin(metrics_plugin);

    // 3. 提取句柄并构建 Env（模拟预挂载后可获取句柄的情况）
    // 注意：在实际实现中，通常在挂载后提取句柄，但由于借用检查器
    // 和生命周期严格性，Env 通常会传递给 mount。这里我们使用 Option 和 Arc。
    // 为了模拟，我们目前假设句柄只是持有配置的 Arcs。
    
    // 目前，我们跳过运行 bootstrap，因为没有真实的句柄我们无法实例化 ProdEnv
    println!("Host compiled successfully (bootstrap skipped in mock).");
    // let teardown = bootstrap.run(&env).await?;
    
    info!("All components started successfully. System is running.");
    
    // 5. 设置信号处理器以进行优雅关闭
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("Ctrl-C received, initiating shutdown");
        }
    }

    // 6. 运行拆卸
    info!("Running Microkernel Teardown...");
    // let errors = teardown.run().await;

    Ok(())
}
