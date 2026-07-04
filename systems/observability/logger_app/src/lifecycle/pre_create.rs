use microkernel_contracts::AppError;

use crate::config::LoggerConfig;

/// 在实例构建之前验证日志记录器配置。
pub fn run(config: &mut LoggerConfig) -> Result<(), AppError> {
    // 将日志级别标准化为小写
    config.level = config.level.to_lowercase();

    let valid_levels = ["trace", "debug", "info", "warn", "error"];
    if !valid_levels.contains(&config.level.as_str()) {
        return Err(AppError::Config(format!(
            "invalid log level '{}'; valid values: {:?}",
            config.level, valid_levels
        )));
    }

    // 如果指定了日志目录，验证其存在或可创建
    if let Some(ref dir) = config.log_dir {
        if !dir.is_empty() {
            std::fs::create_dir_all(dir).map_err(|e| {
                AppError::Config(format!(
                    "log directory '{}' cannot be created: {}",
                    dir, e
                ))
            })?;
        }
    }

    Ok(())
}
