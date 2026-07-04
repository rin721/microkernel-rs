use crate::env::SystemEnv;
use crate::errors::AppError;

/// Business Plugin lifecycle contract.
///
/// Every business plugin (rbac, auth, mfa, metrics, …) **must** implement this
/// trait. Unlike `Archetype` (which manages infrastructure resources), a `Plugin`
/// is a stateful service that interacts with the environment during its lifetime.
///
/// # Hook execution order
///
/// ```text
///   on_load(env)   — plugin receives the assembled environment, performs init
///   on_start(env)  — plugin begins active processing (subscribe to event bus, etc.)
///        ↕  (runtime)
///   on_stop(env)   — plugin ceases processing and drains pending work
///   on_unload(env) — plugin releases all held resources
/// ```
///
/// # Static dispatch
/// Like `Archetype`, `Plugin` uses native `async fn` (RPITIT). All lifecycle
/// calls are statically dispatched through the generic bound `<P: Plugin<E>>`.
pub trait Plugin<E: SystemEnv>: Send + Sync {
    /// Initialize the plugin with a reference to the fully assembled environment.
    ///
    /// Called once, during the bootstrap phase, after all Generic Apps have
    /// completed their `post_mount` hooks. The `env` is fully operational at
    /// this point — the plugin may perform database queries, cache reads, etc.
    ///
    /// # Errors
    /// Return `AppError` to abort bootstrap. The kernel treats a failing `on_load`
    /// as a fatal error and begins teardown immediately.
    fn on_load(&mut self, env: &E) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    /// Begin active processing.
    ///
    /// Use this hook to subscribe to event bus topics, spawn managed background
    /// tasks (via the kernel's task registry), or start accepting requests.
    ///
    /// # Important
    /// **Never** spawn unmanaged `tokio::spawn` tasks here. All background work
    /// must be registered with the kernel so it can be cleanly cancelled during
    /// teardown.
    fn on_start(&mut self, _env: &E) -> impl std::future::Future<Output = Result<(), AppError>> + Send {
        async { Ok(()) }
    }

    /// Cease active processing.
    ///
    /// Cancel any event bus subscriptions and signal managed background tasks
    /// to stop. Wait for in-flight work to drain before returning.
    fn on_stop(&mut self, _env: &E) -> impl std::future::Future<Output = Result<(), AppError>> + Send {
        async { Ok(()) }
    }

    /// Release all resources held by the plugin.
    ///
    /// Called after `on_stop` has returned. At this point the environment may
    /// have begun teardown — do not attempt to access infrastructure resources
    /// (e.g., issue database queries) in this hook.
    fn on_unload(&mut self, _env: &E) -> impl std::future::Future<Output = Result<(), AppError>> + Send {
        async { Ok(()) }
    }

    /// Human-readable name used in log messages and error reports.
    fn name(&self) -> &'static str;
}
