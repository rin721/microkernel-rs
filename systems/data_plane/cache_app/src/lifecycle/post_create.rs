use microkernel_contracts::AppError;
use tracing::info;
use crate::CacheApp;

pub async fn run(app: &CacheApp) -> Result<(), AppError> {
    info!(url = %app.config.redis_url, capacity = app.config.local_capacity, "cache components initialized (simulated)");
    Ok(())
}
