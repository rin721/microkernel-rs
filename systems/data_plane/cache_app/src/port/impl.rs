use std::time::Duration;
use microkernel_contracts::{AppError, CachePort};

pub struct CacheHandle {
    // In a real implementation:
    // pub(crate) redis_pool: ...
    // pub(crate) local_cache: moka::future::Cache<String, Vec<u8>>,
}

impl CachePort for CacheHandle {
    async fn get(&self, _key: &str) -> Result<Option<Vec<u8>>, AppError> {
        Ok(None)
    }

    async fn set(
        &self,
        _key: &str,
        _value: Vec<u8>,
        _ttl: Option<Duration>,
    ) -> Result<(), AppError> {
        Ok(())
    }

    async fn del(&self, _key: &str) -> Result<(), AppError> {
        Ok(())
    }

    async fn ttl(&self, _key: &str) -> Result<Option<Duration>, AppError> {
        Ok(None)
    }

    async fn ping(&self) -> Result<(), AppError> {
        Ok(())
    }
}
