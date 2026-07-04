use crate::env::SystemEnv;
use crate::errors::AppError;

/// Health status reported by a Generic App's `health_check` hook.
///
/// The kernel polls this periodically and exposes the aggregated result through
/// the metrics/observability layer.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum HealthStatus {
    /// The component is operating normally.
    Healthy,
    /// The component is operational but with reduced capability.
    Degraded {
        /// Human-readable explanation of the degradation.
        reason: String,
    },
    /// The component is not operational. The kernel may attempt a restart.
    Unhealthy,
}

/// Generic App lifecycle contract — the kernel's "霸王条款" for all Generic Apps.
///
/// Every Generic App (database, cache, storage, logger, etc.) **must** implement
/// this trait. The kernel calls hooks in strict sequence and will abort the bootstrap
/// sequence on any `Err` returned by a **mandatory** hook.
///
/// # Hook execution order
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
/// # Async in trait
/// Uses Rust 1.75+ native `async fn` in trait (RPITIT). No `async_trait` macro
/// is needed because all callers use static dispatch via `<A: Archetype<E>>`.
pub trait Archetype<E: SystemEnv>: Send + Sync + Sized {
    /// The configuration type for this app.
    ///
    /// Must implement `Default` so the kernel can call `default_config()` when
    /// no explicit configuration is supplied.
    type Config: Send + Sync + Default;

    // ── Configuration ─────────────────────────────────────────────────────────

    /// Return the default configuration for this app.
    ///
    /// The kernel calls this when no explicit config is provided, enabling
    /// "zero-config / batteries-included" startup.
    fn default_config() -> Self::Config {
        Self::Config::default()
    }

    // ── Mandatory hooks (must be implemented; no default body) ─────────────────

    /// Called **before** the app instance is constructed.
    ///
    /// Use this hook to validate and normalize the configuration (e.g., check
    /// that a database URL is well-formed, resolve environment variable
    /// overrides). The `config` parameter is `&mut` so corrections can be
    /// applied in-place.
    ///
    /// # Errors
    /// Return `AppError::Config` if validation fails. The kernel will abort
    /// the bootstrap sequence.
    fn pre_create(config: &mut Self::Config) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    /// Called immediately after the app instance is constructed, before mounting.
    ///
    /// Use this hook to perform post-construction self-checks (e.g., verify
    /// that the connection pool was populated correctly).
    fn post_create(&self) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    /// Called just before the app is registered into the kernel's environment.
    ///
    /// Use this hook for actions that require infrastructure to be partially
    /// available but not yet serving traffic — the canonical use case is running
    /// database migrations.
    fn pre_mount(&self, env: &E) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    /// Called after the app has been registered and the environment is available.
    ///
    /// Use this hook to announce readiness or to perform cross-app wiring
    /// (e.g., subscribe to event bus topics).
    fn post_mount(&self, env: &E) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    // ── Optional hooks (default: no-op `Ok(())`) ─────────────────────────────

    /// Called before the app begins accepting traffic / serving requests.
    fn pre_start(&self, _env: &E) -> impl std::future::Future<Output = Result<(), AppError>> + Send {
        async { Ok(()) }
    }

    /// Called after the app has started and is fully operational.
    fn post_start(&self, _env: &E) -> impl std::future::Future<Output = Result<(), AppError>> + Send {
        async { Ok(()) }
    }

    /// Called before a hot-reload of the configuration is applied.
    ///
    /// Return `Err` to veto the reload (e.g., the new config is invalid).
    fn pre_reload(&self, _new_config: &Self::Config) -> impl std::future::Future<Output = Result<(), AppError>> + Send {
        async { Ok(()) }
    }

    /// Called after the hot-reload has been successfully applied.
    fn post_reload(&self, _new_config: &Self::Config) -> impl std::future::Future<Output = Result<(), AppError>> + Send {
        async { Ok(()) }
    }

    /// Called when the shutdown signal is received, before resources are released.
    ///
    /// Use this hook to stop accepting new requests and wait for in-flight
    /// operations to complete (graceful shutdown).
    fn pre_stop(&self) -> impl std::future::Future<Output = Result<(), AppError>> + Send {
        async { Ok(()) }
    }

    /// Called after all resources have been released.
    ///
    /// Use this hook to emit final telemetry or cleanup notifications.
    fn post_stop(&self) -> impl std::future::Future<Output = Result<(), AppError>> + Send {
        async { Ok(()) }
    }

    /// Report the current health of the component.
    ///
    /// The kernel polls this on a configurable interval and aggregates results
    /// for the `/health` endpoint.
    fn health_check(&self) -> impl std::future::Future<Output = Result<HealthStatus, AppError>> + Send {
        async { Ok(HealthStatus::Healthy) }
    }
}
