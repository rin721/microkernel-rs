use microkernel_contracts::{AppError, DatabasePort};
use microkernel_contracts::ports::{DbRow, DbValue, QueryResult};
use sea_orm::DatabaseConnection;

pub struct DbHandle {
    pub(crate) pool: DatabaseConnection,
}

impl DatabasePort for DbHandle {
    async fn ping(&self) -> Result<(), AppError> {
        // sea_orm::DatabaseConnection::ping 存在
        let _ = self.pool.ping().await.map_err(|e| AppError::Io(e.to_string()))?;
        Ok(())
    }

    async fn fetch_all(
        &self,
        _sql: &str,
        _params: Vec<DbValue>,
    ) -> Result<Vec<DbRow>, AppError> {
        Ok(vec![])
    }

    async fn fetch_one(
        &self,
        _sql: &str,
        _params: Vec<DbValue>,
    ) -> Result<DbRow, AppError> {
        Err(AppError::NotFound("Not implemented".to_owned()))
    }

    async fn execute(
        &self,
        _sql: &str,
        _params: Vec<DbValue>,
    ) -> Result<QueryResult, AppError> {
        Ok(QueryResult {
            rows_affected: 0,
            last_insert_id: None,
        })
    }
}
