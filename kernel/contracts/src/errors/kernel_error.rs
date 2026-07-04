use thiserror::Error;

use crate::errors::AppError;

/// Kernel-level error type for bootstrap and teardown failures.
///
/// Unlike `AppError` (which surfaces from application code), `KernelError` is raised
/// by the microkernel engine itself when lifecycle orchestration fails critically.
///
/// # Stability
/// `#[non_exhaustive]` — see `AppError` docs for rationale.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum KernelError {
    /// A component failed during the bootstrap (Load → Mount → Start) phase.
    #[error("bootstrap failed for component '{component}': {source}")]
    BootstrapFailed {
        /// Human-readable name of the failing component (e.g., `"DatabaseApp"`).
        component: String,
        /// The underlying application error that caused the failure.
        source: AppError,
    },

    /// A component failed during the teardown (PreStop → PostStop) phase.
    ///
    /// Teardown errors are **non-fatal** — the kernel logs them and continues
    /// stopping remaining components. All teardown errors are collected and
    /// reported together at the end.
    #[error("teardown failed for component '{component}': {source}")]
    TeardownFailed {
        component: String,
        source: AppError,
    },

    /// The environment could not be assembled (e.g., a required port is missing).
    #[error("environment build failed: {0}")]
    EnvBuildFailed(String),

    /// The event bus channel is at capacity; the publisher must back off.
    ///
    /// This error is returned from `EventDispatcher::publish` when the bounded
    /// channel buffer is full, surfacing backpressure to the caller explicitly.
    /// **Never silently discarded.**
    #[error("event bus backpressure exceeded: channel capacity reached")]
    BackpressureExceeded,

    /// A task cancellation signal could not be delivered.
    #[error("cancellation signal failed: {0}")]
    CancellationFailed(String),
}
