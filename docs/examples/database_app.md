# 数据库通用应用设计示例 (Database App Example)

本示例展示如何将一个数据库库（例如基于 SQLx 的 PostgreSQL 客户端）封装为微内核架构下的通用应用，通过实现内核下发的接口契约，交由 Host Proxy 代理调度。

## 1. 目标与机制

*   **职责**：为当前系统封装实现数据库应用为通用库（管理连接池）。
*   **接入方式**：作为“通用应用库”，通过实现系统本体在 `kernel/lifecycle` 层定义的 `AppLifecycle` 约束接口，向内核注册。

## 2. 接口实现规范 (Implementation of Kernel Contract)

在 [微内核核心架构设计](../architecture/microkernel.md) 中已明确声明：**生命周期钩子 Trait 是内核的核心设计，属于系统本体用来约束新增库的霸王条款。应用本身不可随意定义，只需按约实现即可。**

下面是数据库应用具体实现内核 `AppLifecycle` 接口的伪代码演示：

```rust
use async_trait::async_trait;
// 从内核模块引入统一生命周期契约
use crate::kernel::lifecycle::{AppLifecycle, AppError}; 

/// 数据库配置定义
pub struct DbConfig {
    pub enabled: bool, // 供内核判断是否启动此应用
    pub url: String,
    pub max_connections: u32,
}

/// 数据库应用实体
pub struct DatabaseApp {
    // 内部封装的连接池等资源
    pub pool: Option<sqlx::PgPool>, 
}

/// 实现系统本体强制约束的生命周期 Trait
#[async_trait]
impl AppLifecycle for DatabaseApp {
    type Config = DbConfig;

    /// 1. 默认配置实现
    fn default_config() -> Self::Config {
        DbConfig {
            enabled: false, 
            url: "postgres://localhost/default".to_string(),
            max_connections: 10,
        }
    }

    /// 2. 实例创建前逻辑
    async fn pre_create(config: &mut Self::Config) -> Result<(), AppError> {
        // 场景：对配置项中的 url 进行连通性格式检查
        tracing::info!("准备创建数据库实例，检测配置合法性...");
        Ok(())
    }

    /// 3. 实例创建后逻辑
    async fn post_create(&self) -> Result<(), AppError> {
        // 场景：打印已初始化日志
        tracing::info!("数据库连接池实例已成功创建");
        Ok(())
    }

    /// 4. 挂载前逻辑
    async fn pre_mount(&self) -> Result<(), AppError> {
        // 场景：在向内核注册表暴漏该应用前，执行强制的自动迁移脚本 (Migrations)
        tracing::info!("执行数据库 Schema Migration...");
        Ok(())
    }

    /// 5. 挂载后逻辑
    async fn post_mount(&self) -> Result<(), AppError> {
        // 场景：通知外界可以开始建立业务连接
        tracing::info!("数据库已挂载至微内核，开放访问");
        Ok(())
    }

    /// (可选) 健康探测逻辑，覆盖默认实现
    async fn health_check(&self) -> Result<HealthStatus, AppError> {
        // 场景：执行 SELECT 1 探测数据库是否存活
        if let Some(pool) = &self.pool {
            // let _ = sqlx::query("SELECT 1").execute(pool).await?;
            tracing::debug!("执行数据库健康检查: OK");
            Ok(HealthStatus::Healthy)
        } else {
            Ok(HealthStatus::Unhealthy)
        }
    }

    /// (可选) 配置重载前校验逻辑
    async fn pre_reload(&self, new_config: &Self::Config) -> Result<(), AppError> {
        // 场景：在应用新配置前检查 URL 是否合法
        tracing::info!("预检查新配置的合法性...");
        Ok(())
    }

    /// 6. 重载逻辑
    async fn post_reload(&self, new_config: &Self::Config) -> Result<(), AppError> {
        // 场景：判断数据库连接池参数是否需要动态扩缩容
        tracing::info!("收到配置重载信号，调整连接池参数...");
        Ok(())
    }

    /// 7. 停止前逻辑
    async fn pre_stop(&self) -> Result<(), AppError> {
        // 场景：向外发出拒绝新请求信号，优雅停机
        tracing::info!("即将关闭数据库应用，等待活动事务结束...");
        Ok(())
    }

    /// 8. 停止后逻辑
    async fn post_stop(&self) -> Result<(), AppError> {
        // 场景：日志记录或告警通知资源已回收
        tracing::info!("数据库应用底层资源已彻底释放");
        Ok(())
    }
}
```

## 3. 被代理调用的结果

当上述代码实现完成后，通用数据库应用就完成了自身的职责闭环。

接下来，系统本体 (Host Proxy) 会在启动时自动读取到 `DatabaseApp` 对应的类型，并通过内部的 `mount_database_app` 等统一工作流编排机制，自动代理触发这些钩子，从而实现对底层框架启停的完全控制。应用开发者无需在 main 函数中手动编排这些调用的顺序。
