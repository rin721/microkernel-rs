use microkernel_contracts::AppError;
use opendal::Error as OpendalError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("opendal error: {0}")]
    Opendal(#[from] OpendalError),
}

impl From<StorageError> for AppError {
    fn from(err: StorageError) -> Self {
        match err {
            StorageError::Opendal(e) => AppError::Io(e.to_string()),
        }
    }
}
