use microkernel_contracts::AppError;
use tracing::info;

use crate::LoggerApp;

/// 发出确认日志记录器已挂载的第一条结构化日志条目。
pub fn run(_app: &LoggerApp) -> Result<(), AppError> {
    info!(
        component = "LoggerApp",
        "logger mounted — structured logging is active"
    );
    Ok(())
}
