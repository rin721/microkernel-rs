use std::collections::HashMap;

use crate::errors::AppError;

// ── Value types ───────────────────────────────────────────────────────────────

/// 数据库查询返回的单个类型化的单元格值。
///
/// 此枚举提供了一个干净的抽象边界，以便业务插件永远
/// 不需要直接依赖于 `sqlx` 或 `sea-orm`。所有特定于基础设施的
/// 类型转换都发生在 `database_app` 内部。
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum DbValue {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
    Bytes(Vec<u8>),
}

/// 从查询返回的单行，由列名键控。
pub type DbRow = HashMap<String, DbValue>;

/// 写操作（INSERT / UPDATE / DELETE）后返回的元数据。
#[derive(Debug, Clone)]
pub struct QueryResult {
    /// 受语句影响的行数。
    pub rows_affected: u64,
    /// 最后插入行的 ID，如果数据库支持并且语句是 INSERT 的话。
    pub last_insert_id: Option<i64>,
}

// ── Port Trait ────────────────────────────────────────────────────────────────

/// 用于关系数据库访问的端口 Trait。
///
/// 业务插件通过 `env.db()` 调用这些方法，而不直接
/// 依赖于 `sqlx`、`sea-orm` 或任何特定的数据库驱动程序。
///
/// # 设计说明
/// 参数使用 `Vec<DbValue>`（位置参数）而不是命名参数，
/// 以保持驱动无关性。`database_app` 中的具体实现会将
/// 它们映射到适当的驱动 API。
pub trait DatabasePort: Send + Sync + 'static {
    /// 验证数据库连接是否处于活跃状态（例如，`SELECT 1`）。
    fn ping(&self) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    /// 执行一个 SELECT 语句并返回所有匹配的行。
    ///
    /// # 参数
    /// * `sql`    — 参数化的 SQL 语句 (使用 `?` 或 `$N` 占位符)
    /// * `params` — 位置参数的值
    fn fetch_all(
        &self,
        sql: &str,
        params: Vec<DbValue>,
    ) -> impl std::future::Future<Output = Result<Vec<DbRow>, AppError>> + Send;

    /// 执行一个 SELECT 语句并恰好返回一行。
    ///
    /// 如果没有匹配的行，返回 `AppError::NotFound`。
    fn fetch_one(
        &self,
        sql: &str,
        params: Vec<DbValue>,
    ) -> impl std::future::Future<Output = Result<DbRow, AppError>> + Send;

    /// 执行一个 INSERT，UPDATE 或 DELETE 语句。
    ///
    /// 返回一个包含受影响行数以及（如果适用）最后插入行 ID 的 [`QueryResult`]。
    fn execute(
        &self,
        sql: &str,
        params: Vec<DbValue>,
    ) -> impl std::future::Future<Output = Result<QueryResult, AppError>> + Send;
}
