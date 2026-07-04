use std::time::Duration;

use crate::errors::AppError;

/// Port Trait for two-level (local + distributed) cache access.
///
/// Implementations route reads through local memory first (L1 cache via `moka`),
/// then fall back to the distributed store (L2 cache via `redis`). Writes are
/// applied to both levels with the specified TTL.
pub trait CachePort: Send + Sync + 'static {
    /// Retrieve a value by key.
    ///
    /// Returns `None` if the key does not exist or has expired.
    fn get(
        &self,
        key: &str,
    ) -> impl std::future::Future<Output = Result<Option<Vec<u8>>, AppError>> + Send;

    /// Store a value with an optional time-to-live.
    ///
    /// If `ttl` is `None` the entry persists until explicitly deleted.
    /// If `ttl` is `Some(Duration)` the entry expires after that duration.
    fn set(
        &self,
        key: &str,
        value: Vec<u8>,
        ttl: Option<Duration>,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    /// Delete a value by key. Succeeds even if the key does not exist.
    fn del(
        &self,
        key: &str,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    /// Return the remaining time-to-live for a key.
    ///
    /// Returns `None` if the key does not exist or has no expiry.
    fn ttl(
        &self,
        key: &str,
    ) -> impl std::future::Future<Output = Result<Option<Duration>, AppError>> + Send;

    /// Verify that the cache backend is reachable (e.g., PING Redis).
    fn ping(&self) -> impl std::future::Future<Output = Result<(), AppError>> + Send;
}
