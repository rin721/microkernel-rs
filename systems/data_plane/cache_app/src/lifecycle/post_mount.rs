use microkernel_contracts::AppError;
use tracing::info;
use crate::CacheApp;

pub async fn run(_app: &CacheApp) -> Result<(), AppError> {
    info!("two-level cache mounted");
    Ok(())
}
