use microkernel_contracts::AppError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("redis error: {0}")]
    Redis(#[from] redis::RedisError),
}

impl From<CacheError> for AppError {
    fn from(err: CacheError) -> Self {
        match err {
            CacheError::Redis(e) => AppError::Io(e.to_string()),
        }
    }
}
