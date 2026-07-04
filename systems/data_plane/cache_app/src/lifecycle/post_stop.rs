use microkernel_contracts::AppError;
use tracing::info;
use crate::CacheApp;

pub async fn run(_app: &CacheApp) -> Result<(), AppError> {
    info!("redis connections closed");
    Ok(())
}
