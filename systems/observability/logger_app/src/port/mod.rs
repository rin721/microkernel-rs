use microkernel_contracts::LoggerPort;

/// 活跃日志子系统的句柄。
///
/// 实现 `LoggerPort`，它是告诉 `SystemEnv` 
/// 结构化日志已配置的编译时标记。
pub struct LoggerHandle;

impl LoggerPort for LoggerHandle {
    async fn flush(&self) {
        // tracing-appender 的 NonBlocking 写入器在
        // （存储在 LoggerApp 中的）WorkerGuard 在 `post_stop` 中被丢弃时自动刷新。
        // 此方法对外部调用者是一个空操作；guard 会处理它。
    }
}
