/// 日志子系统的标记端口 Trait。
///
/// 结构化日志记录是通过 `tracing` crate 的宏（`tracing::info!`、
/// `tracing::warn!` 等）而不是方法调用来访问的。
/// 此 trait 的唯一目的是参与 `SystemEnv` 的关联类型
/// 系统，允许内核在编译时验证在将环境视为有效之前，
/// 已经配置了日志记录器。
///
/// # 在插件中使用
/// 插件**不要**在 `LoggerPort` 上调用方法。它们只需使用标准的
/// `tracing` 宏，它们将自动路由到 `logger_app`
/// 安装为全局默认的任何订阅者。
pub trait LoggerPort: Send + Sync + 'static {
    /// 刷新所有缓冲的日志记录到其支持的文件系统或接收器。
    ///
    /// 由内核在销毁期间调用，以确保所有在处理中的日志条目
    /// 在进程退出前都能到达其目的地。
    fn flush(&self) -> impl std::future::Future<Output = ()> + Send;
}
