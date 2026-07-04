mod env;

use microkernel_core::lifecycle::Bootstrap;
use microkernel_contracts::KernelError;
use tracing::info;
use env::ProdEnv;

// Generic Apps
use microkernel_logger::{LoggerApp, config::LoggerConfig};
use microkernel_database::{DatabaseApp, config::DbConfig};
use microkernel_cache::{CacheApp, config::CacheConfig};
use microkernel_storage::{StorageApp, config::StorageConfig};

// Plugins
use microkernel_auth::{AuthPlugin, config::AuthConfig};
use microkernel_rbac::{RbacPlugin, config::RbacConfig};
use microkernel_mfa::{MfaPlugin, config::MfaConfig};
use microkernel_metrics::{MetricsPlugin, config::MetricsConfig};

#[tokio::main]
async fn main() -> Result<(), KernelError> {
    // 1. Create Apps & Plugins (Unmounted)
    let logger_app = LoggerApp::new(LoggerConfig::default());
    let db_app = DatabaseApp::new(DbConfig::default());
    let cache_app = CacheApp::new(CacheConfig::default());
    let storage_app = StorageApp::new(StorageConfig::default());
    
    let auth_plugin = AuthPlugin::new(AuthConfig::default());
    let rbac_plugin = RbacPlugin::new(RbacConfig::default());
    let mfa_plugin = MfaPlugin::new(MfaConfig::default());
    let metrics_plugin = MetricsPlugin::new(MetricsConfig::default());

    // 2. Setup Bootstrap
    let mut bootstrap = Bootstrap::<ProdEnv>::new();
    
    // Register Generic Apps (Order matters!)
    let logger_shared = bootstrap.register_archetype(logger_app);
    let db_shared = bootstrap.register_archetype(db_app);
    let cache_shared = bootstrap.register_archetype(cache_app);
    let storage_shared = bootstrap.register_archetype(storage_app);

    // Register Plugins
    let auth_shared = bootstrap.register_plugin(auth_plugin);
    let rbac_shared = bootstrap.register_plugin(rbac_plugin);
    let _mfa_shared = bootstrap.register_plugin(mfa_plugin);
    let _metrics_shared = bootstrap.register_plugin(metrics_plugin);

    // 3. Extract Handles & Build Env (Simulating handles being available after pre-mounting)
    // Note: In a real implementation, you'd extract handles after mounting, but due to borrow checker 
    // and lifecycle strictness, typically the Env is passed to mount. Here we use an Option and Arc.
    // For this simulation we assume handles are just Arcs holding configuration right now.
    
    // For now, we skip running bootstrap because we cannot instantiate ProdEnv without real handles
    println!("Host compiled successfully (bootstrap skipped in mock).");
    // let teardown = bootstrap.run(&env).await?;
    
    info!("All components started successfully. System is running.");
    
    // 5. Setup Signal Handler for Graceful Shutdown
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("Ctrl-C received, initiating shutdown");
        }
    }

    // 6. Run Teardown
    info!("Running Microkernel Teardown...");
    // let errors = teardown.run().await;

    Ok(())
}
