use microkernel_contracts::{AppError, StoragePort};
use microkernel_contracts::ports::StorageObject;

pub struct StorageHandle {
    // pub(crate) op: opendal::Operator,
}

impl StoragePort for StorageHandle {
    async fn read(&self, _path: &str) -> Result<Vec<u8>, AppError> {
        Ok(vec![])
    }

    async fn write(
        &self,
        _path: &str,
        _data: Vec<u8>,
        _content_type: Option<&str>,
    ) -> Result<(), AppError> {
        Ok(())
    }

    async fn delete(&self, _path: &str) -> Result<(), AppError> {
        Ok(())
    }

    async fn list(&self, _prefix: &str) -> Result<Vec<StorageObject>, AppError> {
        Ok(vec![])
    }

    async fn exists(&self, _path: &str) -> Result<bool, AppError> {
        Ok(false)
    }
}
