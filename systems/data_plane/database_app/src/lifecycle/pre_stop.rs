use microkernel_contracts::AppError;
use tracing::info;

use crate::DatabaseApp;

pub async fn run(_app: &DatabaseApp) -> Result<(), AppError> {
    info!("database preparing to stop, waiting for active transactions to complete");
    Ok(())
}
