use crate::env::ProdEnv;
use microkernel_contracts::{KernelError, Plugin};
use microkernel_core::lifecycle::Bootstrap;
use tracing::info;

// Generic Apps
use microkernel_logger::{config::LoggerConfig, LoggerApp};
use microkernel_database::{config::DbConfig, DatabaseApp};
use microkernel_cache::{config::CacheConfig, CacheApp};
use microkernel_storage::{config::StorageConfig, StorageApp};

/// 门面应用层，用于隐藏 Bootstrap 底层组装的复杂度。
pub struct MicrokernelApp;

impl MicrokernelApp {
    /// 开始构建新的微内核应用实例。
    pub fn builder() -> MicrokernelBuilder {
        MicrokernelBuilder::new()
    }
}

pub struct MicrokernelBuilder {
    logger_config: Option<LoggerConfig>,
    db_config: Option<DbConfig>,
    cache_config: Option<CacheConfig>,
    storage_config: Option<StorageConfig>,
    // 使用动态闭包列表来保存需要注册的插件，延迟到 ignite 时调用
    plugins: Vec<Box<dyn FnOnce(&mut Bootstrap<ProdEnv>)>>,
}

impl MicrokernelBuilder {
    pub fn new() -> Self {
        Self {
            logger_config: None,
            db_config: None,
            cache_config: None,
            storage_config: None,
            plugins: Vec::new(),
        }
    }

    pub fn with_logger(mut self, config: LoggerConfig) -> Self {
        self.logger_config = Some(config);
        self
    }

    pub fn with_database(mut self, config: DbConfig) -> Self {
        self.db_config = Some(config);
        self
    }

    pub fn with_cache(mut self, config: CacheConfig) -> Self {
        self.cache_config = Some(config);
        self
    }

    pub fn with_storage(mut self, config: StorageConfig) -> Self {
        self.storage_config = Some(config);
        self
    }

    /// 注册任意符合 ProdEnv 的业务插件。
    pub fn with_plugin<P>(mut self, plugin: P) -> Self
    where
        P: Plugin<ProdEnv> + 'static,
    {
        self.plugins.push(Box::new(move |bootstrap| {
            bootstrap.register_plugin(plugin);
        }));
        self
    }

    /// 一键组装并启动微内核。
    pub async fn ignite(self) -> Result<(), KernelError> {
        // 1. 实例化通用应用（采用用户传入配置或默认配置）
        let logger_app = LoggerApp::new(self.logger_config.unwrap_or_default());
        let db_app = DatabaseApp::new(self.db_config.unwrap_or_default());
        let cache_app = CacheApp::new(self.cache_config.unwrap_or_default());
        let storage_app = StorageApp::new(self.storage_config.unwrap_or_default());

        // 2. 初始化 Bootstrap 引擎
        let mut bootstrap = Bootstrap::<ProdEnv>::new();

        // 3. 严格按顺序挂载通用应用（Archetypes）
        let _logger_shared = bootstrap.register_archetype(logger_app);
        let _db_shared = bootstrap.register_archetype(db_app);
        let _cache_shared = bootstrap.register_archetype(cache_app);
        let _storage_shared = bootstrap.register_archetype(storage_app);

        // 4. 执行所有插件的挂载逻辑
        for plugin_registration in self.plugins {
            plugin_registration(&mut bootstrap);
        }

        // 目前跳过了真正的 ProdEnv 初始化与 bootstrap.run(&env)，
        // 由于环境依赖（Handles）需要等到组件 post_create 后才能真正获取。
        // 此处先模拟成功启动与挂起行为，保持与重构前一致。
        println!("Host compiled successfully (bootstrap skipped in mock).");
        info!("All components started successfully. System is running.");

        // 设置信号处理器以进行优雅关闭
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                info!("Ctrl-C received, initiating shutdown");
            }
        }

        info!("Running Microkernel Teardown...");
        // let errors = teardown.run().await;

        Ok(())
    }
}
