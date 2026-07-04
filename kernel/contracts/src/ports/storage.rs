use crate::errors::AppError;

/// 存储对象的元数据。
#[derive(Debug, Clone)]
pub struct StorageObject {
    /// 存储后端的完整路径/键。
    pub path: String,
    /// 对象大小，以字节为单位。
    pub size: u64,
    /// MIME 内容类型，如果已知的话。
    pub content_type: Option<String>,
    /// 最后修改的时间戳 (Unix 纪元秒)。
    pub last_modified: Option<i64>,
}

/// 统一对象存储访问的端口 Trait。
///
/// 抽象了本地文件系统、S3、阿里云 OSS、腾讯云 COS，以及
/// `opendal` 支持的任何其他后端。业务插件完全从
/// 底层云提供商解耦。
pub trait StoragePort: Send + Sync + 'static {
    /// 读取对象的完整内容。
    fn read(
        &self,
        path: &str,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, AppError>> + Send;

    /// 将数据写入对象，覆盖任何现有内容。
    fn write(
        &self,
        path: &str,
        data: Vec<u8>,
        content_type: Option<&str>,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    /// 删除对象。即使路径不存在也会成功。
    fn delete(
        &self,
        path: &str,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    /// 列出路径前缀下的所有对象。
    fn list(
        &self,
        prefix: &str,
    ) -> impl std::future::Future<Output = Result<Vec<StorageObject>, AppError>> + Send;

    /// 检查给定路径上是否存在对象。
    fn exists(
        &self,
        path: &str,
    ) -> impl std::future::Future<Output = Result<bool, AppError>> + Send;
}
