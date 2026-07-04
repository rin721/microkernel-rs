use microkernel_contracts::AppError;
use tracing::info;

use crate::LoggerApp;

/// Signal that no more logs should be written after this point.
pub fn run(_app: &LoggerApp) -> Result<(), AppError> {
    info!(component = "LoggerApp", "logger stopping — flushing buffers");
    Ok(())
}
