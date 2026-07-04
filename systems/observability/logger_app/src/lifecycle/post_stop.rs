use microkernel_contracts::AppError;

use crate::LoggerApp;

/// 存储在 `LoggerApp._guard` 中的 WorkerGuard 在 `LoggerApp` 
/// 自身在拆卸结束时被丢弃时被丢弃。这足以刷新
/// 非阻塞追加器。这个钩子是一个空操作，但记录了契约。
pub fn run(_app: &LoggerApp) -> Result<(), AppError> {
    // 刷新由丢弃 WorkerGuard 隐式处理。
    // 故意留为空操作。
    Ok(())
}
