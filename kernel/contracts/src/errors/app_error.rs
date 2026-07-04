use thiserror::Error;

/// Unified application-layer error type.
///
/// All Generic Apps and Business Plugins **must** convert their internal errors into
/// `AppError` before returning from any lifecycle hook. This ensures the kernel
/// can display human-readable diagnostics without depending on crate-specific types.
///
/// # Stability
/// This enum is `#[non_exhaustive]` to allow adding new variants in minor versions
/// without breaking downstream `match` arms (SemVer §5 — enumeration extensibility).
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AppError {
    /// A configuration field failed validation (e.g., invalid URL, out-of-range value).
    #[error("configuration error: {0}")]
    Config(String),

    /// A resource could not be initialized (e.g., connection pool creation failed).
    #[error("initialization error: {0}")]
    Initialization(String),

    /// An I/O operation failed. The inner message is stringified to avoid leaking
    /// infrastructure-specific error types into the contract boundary.
    #[error("i/o error: {0}")]
    Io(String),

    /// The caller supplied invalid or malformed input data.
    #[error("invalid input: {0}")]
    InvalidInput(String),

    /// An unexpected internal error occurred. Treat as a bug if seen in production.
    #[error("internal error: {0}")]
    Internal(String),

    /// The component is temporarily unavailable (e.g., connection pool exhausted).
    #[error("service unavailable: {0}")]
    Unavailable(String),

    /// A requested resource could not be found.
    #[error("not found: {0}")]
    NotFound(String),

    /// The caller lacks permission to perform the requested action.
    #[error("permission denied: {0}")]
    PermissionDenied(String),
}
