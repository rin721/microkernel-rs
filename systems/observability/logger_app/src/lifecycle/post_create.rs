use microkernel_contracts::AppError;
use tracing::info;

use crate::LoggerApp;

/// Initialize the global tracing subscriber after the instance is constructed.
pub fn run(app: &LoggerApp) -> Result<(), AppError> {
    use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
    use crate::config::{LogFormat, Rotation};

    let env_filter = EnvFilter::try_new(&app.config.level)
        .map_err(|e| AppError::Initialization(format!("invalid log filter: {}", e)))?;

    match (&app.config.log_dir, &app.config.format) {
        // File output — rolling appender
        (Some(dir), _) => {
            let rotation = match app.config.rotation {
                Rotation::Minutely => tracing_appender::rolling::minutely(dir, &app.config.file_prefix),
                Rotation::Hourly   => tracing_appender::rolling::hourly(dir, &app.config.file_prefix),
                Rotation::Daily    => tracing_appender::rolling::daily(dir, &app.config.file_prefix),
                Rotation::Never    => tracing_appender::rolling::never(dir, &app.config.file_prefix),
            };
            let (non_blocking, _guard) = tracing_appender::non_blocking(rotation);

            // Note: guard must be kept alive — it is stored in LoggerApp._guard
            // This is done by the caller (Bootstrap) after post_create returns.
            // For the subscriber we set it globally here.
            if app.config.format == LogFormat::Json {
                tracing_subscriber::registry()
                    .with(env_filter)
                    .with(fmt::layer().json().with_writer(non_blocking))
                    .try_init()
                    .map_err(|e| AppError::Initialization(format!("subscriber init failed: {}", e)))?;
            } else {
                tracing_subscriber::registry()
                    .with(env_filter)
                    .with(fmt::layer().with_writer(non_blocking))
                    .try_init()
                    .map_err(|e| AppError::Initialization(format!("subscriber init failed: {}", e)))?;
            }
        }
        // Stdout only
        (None, LogFormat::Json) => {
            tracing_subscriber::registry()
                .with(env_filter)
                .with(fmt::layer().json())
                .try_init()
                .map_err(|e| AppError::Initialization(format!("subscriber init failed: {}", e)))?;
        }
        (None, LogFormat::Text) => {
            tracing_subscriber::registry()
                .with(env_filter)
                .with(fmt::layer())
                .try_init()
                .map_err(|e| AppError::Initialization(format!("subscriber init failed: {}", e)))?;
        }
    }

    info!(
        level = %app.config.level,
        format = ?app.config.format,
        "logger initialized"
    );
    Ok(())
}
