use microkernel_contracts::AppError;

use crate::config::DbConfig;

pub async fn run(config: &mut DbConfig) -> Result<(), AppError> {
    if !config.url.starts_with("postgres://") && !config.url.starts_with("postgresql://") {
        return Err(AppError::Config(
            "database url must start with 'postgres://' or 'postgresql://'".to_owned(),
        ));
    }
    if config.max_connections == 0 {
        return Err(AppError::Config(
            "database max_connections must be > 0".to_owned(),
        ));
    }
    Ok(())
}
