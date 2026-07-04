use microkernel_contracts::AppError;
use tracing::info;
use crate::StorageApp;

pub async fn run(_app: &StorageApp) -> Result<(), AppError> {
    info!("storage mounted and ready");
    Ok(())
}
