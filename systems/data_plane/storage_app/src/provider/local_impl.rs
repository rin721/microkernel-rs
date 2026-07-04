use microkernel_contracts::AppError;
use opendal::{services::Fs, Operator};
use crate::config::StorageConfig;

pub fn build_operator(config: &StorageConfig) -> Result<Operator, AppError> {
    let builder = Fs::default().root(&config.root);

    let op = Operator::new(builder)
        .map_err(|e| AppError::Initialization(format!("failed to init local fs: {}", e)))?
        .finish();

    Ok(op)
}
