use crate::ports::{AuthPort, CachePort, DatabasePort, LoggerPort, RbacPort, StoragePort};

/// 全局静态环境约束。
///
/// `SystemEnv` 作为一个编译时验证的服务定位器。每个关联
/// 类型必须在构建时解析 —— 在任何热路径上都**没有** `Arc<dyn Trait>` 虚表。
///
/// # 工作原理
/// 1. `host` crate 定义了一个实现此 trait 的具体 `ProdEnv` 结构体。
/// 2. 每个通用应用和业务插件都被参数化为 `SomeApp<E: SystemEnv>`。
/// 3. Rust 的单态化为每个具体的 `E` 生成单个完全内联的调用图。
///
/// # 实现 `SystemEnv`
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
    // ── 关联类型 (在编译时静态解析) ────────────────

    /// 具体的反向关系数据库访问器。
    type Db: DatabasePort;
    /// 具体的两级缓存访问器。
    type Cache: CachePort;
    /// 具体的对象存储访问器。
    type Storage: StoragePort;
    /// 具体的 JWT 身份验证访问器。
    type Auth: AuthPort;
    /// 具体的 RBAC/ABAC 策略引擎。
    type Rbac: RbacPort;
    /// 具体的日志子系统句柄。
    type Logger: LoggerPort;

    // ── 访问器 (零成本 — 内联为直接字段读取) ─────────────────

    /// 访问数据库端口。
    fn db(&self) -> &Self::Db;
    /// 访问缓存端口。
    fn cache(&self) -> &Self::Cache;
    /// 访问存储端口。
    fn storage(&self) -> &Self::Storage;
    /// 访问身份验证端口。
    fn auth(&self) -> &Self::Auth;
    /// 访问 RBAC 端口。
    fn rbac(&self) -> &Self::Rbac;
    /// 访问日志端口。
    fn logger(&self) -> &Self::Logger;
}
