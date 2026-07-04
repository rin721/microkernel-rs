use microkernel_contracts::AppError;
use opendal::{services::S3, Operator};
use crate::config::StorageConfig;

pub fn build_operator(config: &StorageConfig) -> Result<Operator, AppError> {
    let mut builder = S3::default().root(&config.root);
    
    if let Some(ref b) = config.bucket { builder = builder.bucket(b); }
    if let Some(ref r) = config.region { builder = builder.region(r); }
    if let Some(ref e) = config.endpoint { builder = builder.endpoint(e); }
    if let Some(ref ak) = config.access_key { builder = builder.access_key_id(ak); }
    if let Some(ref sk) = config.secret_key { builder = builder.secret_access_key(sk); }

    let op = Operator::new(builder)
        .map_err(|e| AppError::Initialization(format!("failed to init s3: {}", e)))?
        .finish();

    Ok(op)
}
