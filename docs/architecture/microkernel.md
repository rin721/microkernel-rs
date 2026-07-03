# 微内核核心架构设计 (Microkernel Architecture)

本项目采用 Rust 后端微内核架构设计，其核心思想是“应用依赖组装注入”和“插件化”。

## 1. 架构目标

核心目标是将系统划分为“内核 (Kernel)”和“插件/应用 (Plugins/Apps)”两部分：
*   **微内核 (Host Proxy)**：负责依赖组装、生命周期管理、配置分发、统一启停以及资源释放。内核掌握所有接口定义的绝对话语权。
*   **插件与应用**：负责具体的业务逻辑，必须遵守内核定义的接口规范才能接入系统。

## 2. 核心设计：应用依赖组装注入

每个应用（如数据库应用、缓存服务等）都需要由内核进行完整的生命周期与依赖管理。

### 2.1 核心约束：通用应用生命周期契约 (AppLifecycle Trait)

**极其重要**：应用钩子（Hooks）都是微内核在 `kernel/lifecycle` 层专门定义的**核心接口契约**。它们的作用是“约束并代理新增应用通用库”，**绝对不应该由具体应用自身来定义或更改**。所有试图接入内核的通用库都必须实现此 Trait。

```rust
use async_trait::async_trait;

/// 通用应用生命周期契约 (由系统内核定义并强制约束)
#[async_trait]
pub trait AppLifecycle {
    type Config;

    /// 1. 为通用库提供默认配置接口
    /// 系统本体会通过 `启用字段` 判断是否自动代理调用此模块。
    fn default_config() -> Self::Config;

    /// 2. 实例创建前接口 (内核调用)
    async fn pre_create(config: &mut Self::Config) -> Result<(), AppError>;

    /// 3. 实例创建后接口 (内核调用)
    async fn post_create(&self) -> Result<(), AppError>;

    /// 4. 实例挂载前接口 (内核调用)
    async fn pre_mount(&self) -> Result<(), AppError>;

    /// 5. 实例挂载后接口 (内核调用)
    async fn post_mount(&self) -> Result<(), AppError>;

    /// 6. 配置重载后接口 (内核调用)
    /// 当内核检测到配置发生变动且属于本应用时调用
    async fn post_reload(&self, new_config: &Self::Config) -> Result<(), AppError>;

    /// 7. 实例停止前接口 (内核调用)
    async fn pre_stop(&self) -> Result<(), AppError>;

    /// 8. 实例停止后接口 (内核调用)
    async fn post_stop(&self) -> Result<(), AppError>;
}
```

### 2.2 组装代理工作流 (Host Proxy Workflow)

系统本体在启动与运行期间，会严格按照以下顺序自动代理调用各个应用实现的钩子：

1.  **合并配置**：通过调用 `default_config` 和合并用户配置来获取最终参数。
2.  **创建实例**：先调用 `pre_create` 进行准备，然后实例化应用底层资源，接着调用 `post_create`。
3.  **挂载实例**：调用 `pre_mount` 进行如数据库自动迁移等工作，随后将其注册到全局依赖注入容器，最后调用 `post_mount`。
4.  **资源清理**：系统退出时，内核调度执行 `pre_stop` 实现优雅停机，然后执行 `post_stop` 完成日志上报等最后清理工作。

## 3. 插件系统设计 (Plugin System)

系统中的业务交互以插件的形式组织：

*   **生命周期管理**：`Load` (加载/依赖声明), `Start` (启动监听), `Destroy` (资源销毁)。
*   **通信与共享协议**：
    *   **接口/Trait**：插件间通过明确定义的 Rust Trait（动态分发 `Arc<dyn Trait>` 优先，热路径考虑静态）进行交互。
    *   **事件总线 (Event Bus)**：用于解耦不同插件间的异步消息传递。

## 4. 实践示例

具体的接入应用示例，请参见 [数据库通用应用示例](../examples/database_app.md)。
