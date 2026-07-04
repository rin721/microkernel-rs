use microkernel_contracts::AppError;

use crate::LoggerApp;

/// The WorkerGuard stored in `LoggerApp._guard` is dropped when the `LoggerApp`
/// itself is dropped at the end of teardown. This is sufficient to flush the
/// non-blocking appender. This hook is a no-op but documents the contract.
pub fn run(_app: &LoggerApp) -> Result<(), AppError> {
    // Flush is handled implicitly by dropping the WorkerGuard.
    // Intentionally left as no-op.
    Ok(())
}
