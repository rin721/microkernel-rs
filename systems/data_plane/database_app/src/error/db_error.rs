use microkernel_contracts::AppError;
use sea_orm::DbErr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("database error: {0}")]
    SeaOrm(#[from] DbErr),
}

impl From<DbError> for AppError {
    fn from(err: DbError) -> Self {
        match err {
            DbError::SeaOrm(e) => AppError::Io(e.to_string()),
        }
    }
}
