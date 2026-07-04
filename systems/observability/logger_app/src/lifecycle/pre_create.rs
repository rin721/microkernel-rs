use microkernel_contracts::AppError;

use crate::config::LoggerConfig;

/// Validate the logger configuration before the instance is constructed.
pub fn run(config: &mut LoggerConfig) -> Result<(), AppError> {
    // Normalize log level to lowercase
    config.level = config.level.to_lowercase();

    let valid_levels = ["trace", "debug", "info", "warn", "error"];
    if !valid_levels.contains(&config.level.as_str()) {
        return Err(AppError::Config(format!(
            "invalid log level '{}'; valid values: {:?}",
            config.level, valid_levels
        )));
    }

    // If a log directory is specified, verify it exists or can be created
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
