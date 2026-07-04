use crate::errors::AppError;

/// Metadata for a stored object.
#[derive(Debug, Clone)]
pub struct StorageObject {
    /// Full path/key within the storage backend.
    pub path: String,
    /// Object size in bytes.
    pub size: u64,
    /// MIME content-type, if known.
    pub content_type: Option<String>,
    /// Last-modified timestamp (Unix epoch seconds).
    pub last_modified: Option<i64>,
}

/// Port Trait for unified object storage access.
///
/// Abstracts local filesystem, S3, Alibaba OSS, Tencent COS, and any other
/// backend supported by `opendal`. Business plugins are completely decoupled
/// from the underlying cloud provider.
pub trait StoragePort: Send + Sync + 'static {
    /// Read the entire contents of an object.
    fn read(
        &self,
        path: &str,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, AppError>> + Send;

    /// Write data to an object, overwriting any existing content.
    fn write(
        &self,
        path: &str,
        data: Vec<u8>,
        content_type: Option<&str>,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    /// Delete an object. Succeeds even if the path does not exist.
    fn delete(
        &self,
        path: &str,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    /// List all objects under a path prefix.
    fn list(
        &self,
        prefix: &str,
    ) -> impl std::future::Future<Output = Result<Vec<StorageObject>, AppError>> + Send;

    /// Check whether an object exists at the given path.
    fn exists(
        &self,
        path: &str,
    ) -> impl std::future::Future<Output = Result<bool, AppError>> + Send;
}
