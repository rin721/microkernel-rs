use microkernel_contracts::LoggerPort;

/// A handle to the active logging subsystem.
///
/// Implements `LoggerPort` which is the compile-time marker that tells
/// `SystemEnv` that structured logging is configured.
pub struct LoggerHandle;

impl LoggerPort for LoggerHandle {
    async fn flush(&self) {
        // tracing-appender's NonBlocking writer is flushed automatically when
        // the WorkerGuard (stored in LoggerApp) is dropped in `post_stop`.
        // This method is a no-op for external callers; the guard handles it.
    }
}
