# 数据库通用应用设计示例 (Database App Example)

本示例展示如何将一个数据库库（例如基于 SQLx 的 PostgreSQL 客户端）封装为微内核架构下的通用应用，并实现完整的生命周期管理（由 Host Proxy 代理调度）。

## 1. 目标与机制

*   **职责**：为当前系统封装实现数据库应用为通用库。
*   **接入方式**：作为“通用应用库”，向系统本体注册其生命周期钩子，系统本体 (Host Proxy) 负责代理调度这些接口。

## 2. 核心 Trait 设计 (Host 本体接口约束)

**极其重要**：应用钩子（Hooks）都是项目微内核（Host Proxy）在 `kernel/lifecycle` 层专门定义的**接口契约**。它们的作用是“约束并代理新增应用通用库”，**绝对不应该由具体应用自身来随意定义或更改**。

通用库应用（如数据库）必须实现这套由系统本体下发的 `AppLifecycle` 约束。系统本体在启动与运行期间，会严格按照以下顺序自动代理调用各个钩子。

```rust
use async_trait::async_trait;

/// 数据库配置定义
pub struct DbConfig {
    pub enabled: bool, // 启用字段
    pub url: String,
    pub max_connections: u32,
}

/// 通用应用生命周期契约
#[async_trait]
pub trait AppLifecycle {
    type Config;

    /// 1. 为通用库提供默认配置接口
    /// 系统本体会通过 `启用字段` (如 config.enabled) 判断是否自动代理调用此模块。
    /// 需定义启用字段判断逻辑：如果系统启动时未配置当前应用或缺失字段，
    /// 本体将调用此接口获取默认配置，并覆盖未配置字段。
    fn default_config() -> Self::Config;

    /// 2. 实例创建前接口
    /// 本体在实例化数据库对象前调用。
    /// 场景：用于校验配置项的合法性，或从云端/密码中心拉取动态凭证。
    async fn pre_create(config: &mut Self::Config) -> Result<(), AppError>;

    /// 3. 实例创建后接口
    /// 本体在底层资源（如数据库连接池对象）实例化完成之后立即调用。
    /// 场景：进行基础的 ping 测试以确认网络通畅。
    async fn post_create(&self) -> Result<(), AppError>;

    /// 4. 实例挂载前接口
    /// 在数据库实例即将被注入到系统的全局注册表前调用。
    /// 场景：执行数据库的自动迁移 (Migrations) 脚本，确保 Schema 达到最新状态。
    async fn pre_mount(&self) -> Result<(), AppError>;

    /// 5. 实例挂载后接口
    /// 实例已挂载到全局注册表（依赖注入容器）。
    /// 场景：通知其他依赖于该数据库的业务插件，数据库已准备好接受请求。
    async fn post_mount(&self) -> Result<(), AppError>;

    /// 6. 配置重载后接口
    /// 系统配置管理数据发生变化时由本体调用。
    /// 场景：本体传入 Diff，应用判断如果变化的数据是当前 `数据库应用` 的配置，
    /// 则触发连接池动态扩缩容或平滑替换底层连接对象。
    async fn post_reload(&self, new_config: &Self::Config) -> Result<(), AppError>;

    /// 7. 实例停止前接口
    /// 系统接收到退出信号，本体在卸载应用前调用。
    /// 场景：优雅停机，向外发出拒绝新请求信号，等待长事务执行完毕。
    async fn pre_stop(&self) -> Result<(), AppError>;

    /// 8. 实例停止后接口
    /// 实例从注册表中移除，底层资源已回收后调用。
    /// 场景：用于记录清理日志、上报监控状态。
    async fn post_stop(&self) -> Result<(), AppError>;
}
```

## 3. 本体代理调用工作流 (Host Proxy Execution Flow)

系统本体 (Host Proxy) 中的加载器会自动对注册的应用执行如下流程：

```rust
// 【伪代码】系统本体 (Host) 代理执行生命周期的过程

async fn mount_database_app(user_config: Option<DbConfig>) -> Result<(), AppError> {
    // 1. 获取并合并默认配置 (通过启用字段判断)
    let mut config = if let Some(c) = user_config {
        if !c.enabled { return Ok(()); } // 未启用，跳过
        c // 可在此处做缺失字段的 Merge 逻辑
    } else {
        DbApp::default_config() 
    };

    // 2. 实例创建前 (Pre-Create)
    DbApp::pre_create(&mut config).await?;

    // -- 本体执行实例创建 (如建立连接池) --
    let db_app_instance = DbApp::new(&config).await?;

    // 3. 实例创建后 (Post-Create)
    db_app_instance.post_create().await?;

    // 4. 挂载前 (Pre-Mount: 例如执行 Migration)
    db_app_instance.pre_mount().await?;

    // -- 本体将实例挂载到注册表 (Registry) --
    KERNEL_REGISTRY.register(db_app_instance.clone());

    // 5. 挂载后 (Post-Mount)
    db_app_instance.post_mount().await?;

    Ok(())
}
```

> [!NOTE]
> 这种强制生命周期代理设计，使得数据库应用、Redis 应用等基础设施被完美拉平，无需在各自的代码中处理启动顺序，全部由内核统一编排。
