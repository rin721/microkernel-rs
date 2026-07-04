use microkernel_contracts::AppError;
use tracing::info;

use crate::LoggerApp;

/// 发出信号表明此后不应再写入日志。
pub fn run(_app: &LoggerApp) -> Result<(), AppError> {
    info!(component = "LoggerApp", "logger stopping — flushing buffers");
    Ok(())
}
