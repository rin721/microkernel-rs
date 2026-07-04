use microkernel_contracts::AppError;
use tracing::info;

use crate::LoggerApp;

/// 挂载前验证对日志目录的写入权限。
pub fn run(app: &LoggerApp) -> Result<(), AppError> {
    if let Some(ref dir) = app.config.log_dir {
        let test_path = std::path::Path::new(dir).join(".write_test");
        std::fs::write(&test_path, b"ok").map_err(|e| {
            AppError::Initialization(format!(
                "log directory '{}' is not writable: {}",
                dir, e
            ))
        })?;
        std::fs::remove_file(&test_path).ok();
        info!(log_dir = %dir, "log directory write permission verified");
    }
    Ok(())
}
