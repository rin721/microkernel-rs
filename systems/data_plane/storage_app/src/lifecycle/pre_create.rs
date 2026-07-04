use microkernel_contracts::AppError;
use crate::config::{StorageBackend, StorageConfig};

pub async fn run(config: &mut StorageConfig) -> Result<(), AppError> {
    match config.backend {
        StorageBackend::S3 | StorageBackend::Oss => {
            if config.bucket.is_none() {
                return Err(AppError::Config("bucket must be specified for cloud storage".to_owned()));
            }
        }
        StorageBackend::Local => {
            // For local, ensure root isn't empty
            if config.root.is_empty() {
                return Err(AppError::Config("root cannot be empty for local storage".to_owned()));
            }
        }
    }
    Ok(())
}
