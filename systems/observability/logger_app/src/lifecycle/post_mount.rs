use microkernel_contracts::AppError;
use tracing::info;

use crate::LoggerApp;

/// Emit the first structured log entry confirming the logger is mounted.
pub fn run(_app: &LoggerApp) -> Result<(), AppError> {
    info!(
        component = "LoggerApp",
        "logger mounted — structured logging is active"
    );
    Ok(())
}
