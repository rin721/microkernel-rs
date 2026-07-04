use std::time::Duration;

use crate::errors::AppError;

/// 针对两级（本地 + 分布式）缓存访问的端口 Trait。
///
/// 实现首先通过本地内存（L1 缓存，通过 `moka`）路由读取，
/// 然后回退到分布式存储（L2 缓存，通过 `redis`）。写入
/// 连同指定的 TTL 会应用于两个级别。
pub trait CachePort: Send + Sync + 'static {
    /// 通过键检索值。
    ///
    /// 如果键不存在或已过期，则返回 `None`。
    fn get(
        &self,
        key: &str,
    ) -> impl std::future::Future<Output = Result<Option<Vec<u8>>, AppError>> + Send;

    /// 存储具有可选生存时间（TTL）的值。
    ///
    /// 如果 `ttl` 是 `None`，条目将持久保存，直到明确删除。
    /// 如果 `ttl` 是 `Some(Duration)`，条目在指定的时间段之后过期。
    fn set(
        &self,
        key: &str,
        value: Vec<u8>,
        ttl: Option<Duration>,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    /// 通过键删除值。即使键不存在也会成功。
    fn del(
        &self,
        key: &str,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    /// 返回键的剩余生存时间。
    ///
    /// 如果键不存在或没有过期时间，则返回 `None`。
    fn ttl(
        &self,
        key: &str,
    ) -> impl std::future::Future<Output = Result<Option<Duration>, AppError>> + Send;

    /// 验证缓存后端是否可访问（例如，PING Redis）。
    fn ping(&self) -> impl std::future::Future<Output = Result<(), AppError>> + Send;
}
