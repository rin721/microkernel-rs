/// Marker Port Trait for the logging subsystem.
///
/// Structured logging is accessed through the `tracing` crate's macros
/// (`tracing::info!`, `tracing::warn!`, etc.) rather than method calls.
/// This trait's sole purpose is to participate in the `SystemEnv` associated-type
/// system, allowing the kernel to verify at compile time that a logger has been
/// configured before the environment is considered valid.
///
/// # Usage in plugins
/// Plugins do **not** call methods on `LoggerPort`. They simply use the standard
/// `tracing` macros, which automatically route through whatever subscriber
/// `logger_app` installed as the global default.
pub trait LoggerPort: Send + Sync + 'static {
    /// Flush any buffered log records to their backing sinks.
    ///
    /// Called by the kernel during teardown to ensure all in-flight log entries
    /// reach their destination before the process exits.
    fn flush(&self) -> impl std::future::Future<Output = ()> + Send;
}
