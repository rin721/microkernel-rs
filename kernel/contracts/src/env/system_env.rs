use crate::ports::{AuthPort, CachePort, DatabasePort, LoggerPort, RbacPort, StoragePort};

/// The global static environment constraint.
///
/// `SystemEnv` acts as a compile-time verified service locator. Every associated
/// type must be resolved at build time — there are **no** `Arc<dyn Trait>` vtables
/// on any hot path.
///
/// # How it works
/// 1. The `host` crate defines a concrete `ProdEnv` struct that implements this trait.
/// 2. Every Generic App and Business Plugin is parameterized as `SomeApp<E: SystemEnv>`.
/// 3. Rust's monomorphization produces a single, fully inlined call graph per concrete `E`.
///
/// # Implementing `SystemEnv`
/// ```rust,ignore
/// use microkernel_contracts::SystemEnv;
///
/// #[derive(Clone)]
/// pub struct ProdEnv {
///     db:      MyDatabaseImpl,
///     cache:   MyCacheImpl,
///     // ...
/// }
///
/// impl SystemEnv for ProdEnv {
///     type Db      = MyDatabaseImpl;
///     type Cache   = MyCacheImpl;
///     // ...
///     fn db(&self)    -> &Self::Db    { &self.db }
///     fn cache(&self) -> &Self::Cache { &self.cache }
///     // ...
/// }
/// ```
pub trait SystemEnv: Clone + Send + Sync + 'static {
    // ── Associated types (statically resolved at compile time) ────────────────

    /// The concrete relational database accessor.
    type Db: DatabasePort;
    /// The concrete two-level cache accessor.
    type Cache: CachePort;
    /// The concrete object storage accessor.
    type Storage: StoragePort;
    /// The concrete JWT authentication accessor.
    type Auth: AuthPort;
    /// The concrete RBAC/ABAC policy engine.
    type Rbac: RbacPort;
    /// The concrete logging subsystem handle.
    type Logger: LoggerPort;

    // ── Accessors (zero-cost — inlined to direct field reads) ─────────────────

    /// Access the database port.
    fn db(&self) -> &Self::Db;
    /// Access the cache port.
    fn cache(&self) -> &Self::Cache;
    /// Access the storage port.
    fn storage(&self) -> &Self::Storage;
    /// Access the auth port.
    fn auth(&self) -> &Self::Auth;
    /// Access the RBAC port.
    fn rbac(&self) -> &Self::Rbac;
    /// Access the logger port.
    fn logger(&self) -> &Self::Logger;
}
