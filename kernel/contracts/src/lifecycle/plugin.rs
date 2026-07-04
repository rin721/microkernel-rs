use crate::env::SystemEnv;
use crate::errors::AppError;

/// 业务插件生命周期契约。
///
/// 每一个业务插件（rbac, auth, mfa, metrics, …）**必须**实现此
/// trait。与 `Archetype`（管理基础设施资源）不同，`Plugin`
/// 是一个有状态的服务，在其生命周期内与环境进行交互。
///
/// # 钩子执行顺序
///
/// ```text
///   on_load(env)   — 插件接收已组装的环境，执行初始化
///   on_start(env)  — 插件开始主动处理（订阅事件总线等）
///        ↕  (runtime)
///   on_stop(env)   — 插件停止处理并排空待处理的工作
///   on_unload(env) — 插件释放所有持有的资源
/// ```
///
/// # 静态分发
/// 像 `Archetype` 一样，`Plugin` 使用原生的 `async fn` (RPITIT)。所有生命周期
/// 调用都是通过泛型边界 `<P: Plugin<E>>` 静态分发的。
pub trait Plugin<E: SystemEnv>: Send + Sync {
    /// 接收完整组装的环境引用以初始化插件。
    ///
    /// 在引导阶段调用一次，在所有通用应用程序完成
    /// 其 `post_mount` 钩子之后。此时 `env` 是完全运行的——
    /// 插件可以执行数据库查询，缓存读取等。
    ///
    /// # 错误
    /// 返回 `AppError` 以中止引导。内核将失败的 `on_load`
    /// 视为致命错误，并立即开始销毁。
    fn on_load(&mut self, env: &E) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    /// 开始主动处理。
    ///
    /// 使用此钩子订阅事件总线主题，生成受管后台
    /// 任务（通过内核的任务注册表），或开始接受请求。
    ///
    /// # 重要
    /// **绝不能**在此处生成不受管的 `tokio::spawn` 任务。所有后台工作
    /// 必须在内核注册，以便在销毁期间干净地取消。
    fn on_start(&mut self, _env: &E) -> impl std::future::Future<Output = Result<(), AppError>> + Send {
        async { Ok(()) }
    }

    /// 停止主动处理。
    ///
    /// 取消任何事件总线订阅并向受管后台任务发出信号
    /// 以停止。在返回之前等待进行中的工作排空。
    fn on_stop(&mut self, _env: &E) -> impl std::future::Future<Output = Result<(), AppError>> + Send {
        async { Ok(()) }
    }

    /// 释放插件持有的所有资源。
    ///
    /// 在 `on_stop` 返回后调用。此时环境可能
    /// 已开始销毁——在此钩子中不要尝试访问基础设施资源
    /// （例如，发出数据库查询）。
    fn on_unload(&mut self, _env: &E) -> impl std::future::Future<Output = Result<(), AppError>> + Send {
        async { Ok(()) }
    }

    /// 用于日志消息和错误报告的人类可读名称。
    fn name(&self) -> &'static str;
}
