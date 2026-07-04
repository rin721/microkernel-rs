# 微内核核心架构设计 (Microkernel Architecture)

本项目采用 Rust 后端微内核架构设计，其核心思想是“应用静态环境组装”和“插件化”。为了追求极致的内存安全与零成本抽象（Zero-Cost Abstraction），系统全面采用**泛型关联类型与环境约束 (Environment Trait)** 的静态分发设计，摒弃了传统的运行时动态依赖注入。

## 1. 架构目标

核心目标是将系统划分为“内核 (Kernel)”和“插件/应用 (Plugins/Apps)”两部分：
*   **微内核 (Host Proxy)**：负责定义全局的环境约束（Environment），并负责组装依赖、管理应用生命周期。内核掌握所有接口定义的绝对话语权。
*   **插件与应用**：负责具体的业务逻辑，必须遵守内核定义的接口规范，并通过泛型绑定到特定的环境中才能运行。

## 2. 核心设计：环境约束与静态生命周期

每个应用（如数据库应用、缓存服务等）都需要由内核进行完整的生命周期与依赖管理。为了达到极限并发性能，系统不使用 `Arc<dyn Trait>` 进行动态多态分发。

### 2.1 核心环境定义 (Environment Trait)

系统定义了一个全局的 `SystemEnv` Trait，它像一个静态的服务目录，包含了所有基础设施抽象接口的关联类型。

```rust
/// 全局环境约束定义，要求所有依赖项在编译期静态决议
pub trait SystemEnv: Clone + Send + Sync + 'static {
    type Db: DatabaseConnection;     // 静态决议的具体数据库类型
    type Cache: CacheService;        // 静态决议的具体缓存类型
    // ... 其他基础设施类型约束
}
```

### 2.2 核心约束：通用应用生命周期契约 (Archetype Trait)

应用钩子（Hooks）都是微内核在 `kernel/lifecycle` 层专门定义的核心接口契约。与传统 DI 框架不同，插件结构体必须带有环境泛型 `<E: SystemEnv>`。

```rust
use async_trait::async_trait;

/// 通用应用生命周期契约 (由系统内核定义并强制约束)
#[async_trait]
pub trait Archetype<E: SystemEnv> {
    type Config;

    /// 1. 为通用库提供默认配置接口
    fn default_config() -> Self::Config;

    /// 2. 实例创建前接口 (内核调用)
    async fn pre_create(config: &mut Self::Config) -> Result<(), AppError>;

    /// 3. 实例创建后接口 (内核调用)
    async fn post_create(&self) -> Result<(), AppError>;

    /// 4. 实例挂载前接口 (内核调用)
    async fn pre_mount(&self, env: &E) -> Result<(), AppError>;

    /// 5. 实例挂载后接口 (内核调用)
    async fn post_mount(&self, env: &E) -> Result<(), AppError>;

    /// 6. 实例启动前接口
    async fn pre_start(&self, env: &E) -> Result<(), AppError> { Ok(()) }

    /// 7. 实例启动后接口
    async fn post_start(&self, env: &E) -> Result<(), AppError> { Ok(()) }

    // ... 其他生命周期钩子（reload, stop 等，类似地传递 env）
}
```

### 2.3 组装代理工作流 (Host Proxy Workflow)

1.  **实例化环境**：内核首先在 `main` 阶段构建出具体的结构体类型（例如 `ProdEnv`），它实现了 `SystemEnv` 并持有了底层的真实连接池。
2.  **生命周期调度**：内核按照顺序调用所有挂载在 `ProdEnv` 上的插件（例如 `UserPlugin<ProdEnv>`）的生命周期钩子，将环境实例的引用传递进去。
3.  **内联优化**：由于所有调用路径都是具体类型之间的直接调用，Rust 编译器会在编译期进行单态化（Monomorphization）并大量运用内联优化，彻底消除虚函数表开销。

## 3. 插件系统设计 (Plugin System)

系统中的业务交互以插件的形式组织：

*   **泛型绑定 (Generic Binding)**：
    *   插件不持有 `Arc<dyn Trait>`，而是在结构体定义时声明其所在的环境。
    ```rust
    pub struct UserPlugin<E: SystemEnv> {
        env: E, // 持有对环境的引用或拥有权
    }
    ```
*   **依赖获取**：插件内部需要使用数据库时，直接调用 `self.env.db().query()`。这会被编译器优化为对具体底层结构体的直接调用。
*   **通信与共享协议**：
    *   **静态分发优先**：插件间交互也倾向于泛型传递，避免动态擦除。
    *   **Actor 消息总线隔离**：如果某个插件的故障容忍度极低，或是极高风险的外部网络操作，则通过基于所有权转移的通道（MPSC Channel）与其他组件隔离通信。

## 4. 实践示例

具体的接入应用示例，请参见 [数据库通用应用示例](../examples/database_app.md)。
