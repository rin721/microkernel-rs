use std::collections::HashMap;

use crate::errors::AppError;

// ── Value types ───────────────────────────────────────────────────────────────

/// A single typed cell value returned by a database query.
///
/// This enum provides a clean abstraction boundary so that business plugins never
/// need to depend on `sqlx` or `sea-orm` directly. All infrastructure-specific
/// type conversions happen inside `database_app`.
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

/// A single row returned from a query, keyed by column name.
pub type DbRow = HashMap<String, DbValue>;

/// Metadata returned after a write operation (INSERT / UPDATE / DELETE).
#[derive(Debug, Clone)]
pub struct QueryResult {
    /// Number of rows affected by the statement.
    pub rows_affected: u64,
    /// Last inserted row ID, if the database supports it and the statement was an INSERT.
    pub last_insert_id: Option<i64>,
}

// ── Port Trait ────────────────────────────────────────────────────────────────

/// Port Trait for relational database access.
///
/// Business plugins call these methods through `env.db()` without any direct
/// dependency on `sqlx`, `sea-orm`, or any specific database driver.
///
/// # Design note
/// Parameters use `Vec<DbValue>` (positional) rather than named parameters to
/// stay driver-agnostic. The concrete implementation in `database_app` maps
/// them to the appropriate driver API.
pub trait DatabasePort: Send + Sync + 'static {
    /// Verify that the database connection is alive (e.g., `SELECT 1`).
    fn ping(&self) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    /// Execute a SELECT statement and return all matching rows.
    ///
    /// # Arguments
    /// * `sql`    — parameterized SQL statement (use `?` or `$N` placeholders)
    /// * `params` — positional parameter values
    fn fetch_all(
        &self,
        sql: &str,
        params: Vec<DbValue>,
    ) -> impl std::future::Future<Output = Result<Vec<DbRow>, AppError>> + Send;

    /// Execute a SELECT statement and return exactly one row.
    ///
    /// Returns `AppError::NotFound` if zero rows match.
    fn fetch_one(
        &self,
        sql: &str,
        params: Vec<DbValue>,
    ) -> impl std::future::Future<Output = Result<DbRow, AppError>> + Send;

    /// Execute an INSERT, UPDATE, or DELETE statement.
    ///
    /// Returns a [`QueryResult`] containing rows affected and (if applicable) the
    /// last inserted row ID.
    fn execute(
        &self,
        sql: &str,
        params: Vec<DbValue>,
    ) -> impl std::future::Future<Output = Result<QueryResult, AppError>> + Send;
}
