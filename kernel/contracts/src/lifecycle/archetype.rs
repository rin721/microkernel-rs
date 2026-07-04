use crate::env::SystemEnv;
use crate::errors::AppError;

/// 通用应用程序 `health_check` 钩子报告的健康状态。
///
/// 内核定期轮询此项并通过指标/可观测性层
/// 公开聚合结果。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum HealthStatus {
    /// 组件运行正常。
    Healthy,
    /// 组件可运行，但功能降低。
    Degraded {
        /// 降低功能的人类可读说明。
        reason: String,
    },
    /// 组件无法运行。内核可能会尝试重启。
    Unhealthy,
}

/// 通用应用生命周期契约 — 内核对所有通用应用的"霸王条款"。
///
/// 每一个通用应用（数据库、缓存、存储、日志等）**必须**实现
/// 此 trait。内核严格按顺序调用钩子，并且在任何**强制性**钩子返回 `Err`
/// 时中止引导序列。
///
/// # 钩子执行顺序
///
/// ```text
///   ┌─ Bootstrap ──────────────────────────────────────────────────────────┐
///   │  pre_create(config) → [construct instance] → post_create()          │
///   │  → pre_mount(env) → post_mount(env)                                 │
///   │  → pre_start(env) → post_start(env)                                 │
///   └──────────────────────────────────────────────────────────────────────┘
///                       ↕  (runtime)
///   ┌─ Reload (optional) ──────────────────────────────────────────────────┐
///   │  pre_reload(new_config) → [apply new config] → post_reload          │
///   └──────────────────────────────────────────────────────────────────────┘
///                       ↕  (shutdown signal)
///   ┌─ Teardown ───────────────────────────────────────────────────────────┐
///   │  pre_stop() → post_stop()                                           │
///   └──────────────────────────────────────────────────────────────────────┘
/// ```
///
/// # Trait 中的 Async
/// 使用 Rust 1.75+ 原生的 trait 内 `async fn` (RPITIT)。不需要 `async_trait` 宏，
/// 因为所有的调用者都通过 `<A: Archetype<E>>` 使用静态分发。
pub trait Archetype<E: SystemEnv>: Send + Sync + Sized {
    /// 此应用程序的配置类型。
    ///
    /// 必须实现 `Default`，以便当没有提供显式配置时，内核可以调用 `default_config()`。
    type Config: Send + Sync + Default;

    // ── 配置 ─────────────────────────────────────────────────────────

    /// 返回此应用程序的默认配置。
    ///
    /// 当没有提供显式配置时，内核将调用此方法，从而实现
    /// “零配置 / 开箱即用” 启动。
    fn default_config() -> Self::Config {
        Self::Config::default()
    }

    // ── 强制性钩子 (必须实现；没有默认实现) ─────────────────

    /// 在构造应用程序实例**之前**调用。
    ///
    /// 使用此钩子来验证和标准化配置（例如，检查
    /// 数据库 URL 是否格式正确，解析环境变量覆盖）。
    /// `config` 参数是 `&mut`，因此可以在原地应用更正。
    ///
    /// # 错误
    /// 如果验证失败，返回 `AppError::Config`。内核将中止引导序列。
    fn pre_create(config: &mut Self::Config) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    /// 在构造应用程序实例后，装载之前立即调用。
    ///
    /// 使用此钩子执行构建后的自检（例如，验证连接池是否已正确填充）。
    fn post_create(&self) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    /// 在应用程序注册到内核的环境之前调用。
    ///
    /// 使用此钩子进行那些基础设施需要部分可用，但尚未开始提供流量操作的操作——典型的用例是运行数据库迁移。
    fn pre_mount(&self, env: &E) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    /// 在应用程序已注册并且环境可用后调用。
    ///
    /// 使用此钩子宣布就绪或执行跨应用程序连接（例如，订阅事件总线主题）。
    fn post_mount(&self, env: &E) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    // ── 可选钩子 (默认: 无操作 `Ok(())`) ─────────────────────────────

    /// 在应用程序开始接受流量/服务请求之前调用。
    fn pre_start(&self, _env: &E) -> impl std::future::Future<Output = Result<(), AppError>> + Send {
        async { Ok(()) }
    }

    /// 在应用程序启动并完全运行之后调用。
    fn post_start(&self, _env: &E) -> impl std::future::Future<Output = Result<(), AppError>> + Send {
        async { Ok(()) }
    }

    /// 在应用配置的热重载之前调用。
    ///
    /// 返回 `Err` 以否决重载（例如，新配置无效）。
    fn pre_reload(&self, _new_config: &Self::Config) -> impl std::future::Future<Output = Result<(), AppError>> + Send {
        async { Ok(()) }
    }

    /// 在成功应用热重载后调用。
    fn post_reload(&self, _new_config: &Self::Config) -> impl std::future::Future<Output = Result<(), AppError>> + Send {
        async { Ok(()) }
    }

    /// 收到关机信号时，在释放资源之前调用。
    ///
    /// 使用此钩子停止接受新请求，并等待进行中的操作完成（优雅关机）。
    fn pre_stop(&self) -> impl std::future::Future<Output = Result<(), AppError>> + Send {
        async { Ok(()) }
    }

    /// 在所有资源被释放之后调用。
    ///
    /// 使用此钩子发出最终的遥测数据或清理通知。
    fn post_stop(&self) -> impl std::future::Future<Output = Result<(), AppError>> + Send {
        async { Ok(()) }
    }

    /// 报告组件的当前健康状态。
    ///
    /// 内核按照可配置的时间间隔轮询此状态，并为 `/health` 端点聚合结果。
    fn health_check(&self) -> impl std::future::Future<Output = Result<HealthStatus, AppError>> + Send {
        async { Ok(HealthStatus::Healthy) }
    }
}
