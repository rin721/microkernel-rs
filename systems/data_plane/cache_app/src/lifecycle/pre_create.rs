use microkernel_contracts::AppError;
use crate::config::CacheConfig;

pub async fn run(config: &mut CacheConfig) -> Result<(), AppError> {
    if !config.redis_url.starts_with("redis://") && !config.redis_url.starts_with("rediss://") {
        return Err(AppError::Config(
            "redis_url must start with 'redis://' or 'rediss://'".to_owned(),
        ));
    }
    Ok(())
}
