use std::time::Duration;

use microkernel_contracts::AppError;
use sea_orm::ConnectOptions;
use tracing::info;

use crate::DatabaseApp;

pub async fn run(app: &DatabaseApp) -> Result<(), AppError> {
    // 在实际实现中，我们将在此处建立连接池
    // 并使用内部可变性或 unsafe cell 将其分配给 app.pool。
    // 对于本演示，我们只记录日志。
    let mut opt = ConnectOptions::new(app.config.url.clone());
    opt.max_connections(app.config.max_connections)
        .min_connections(app.config.min_connections)
        .connect_timeout(Duration::from_secs(app.config.connect_timeout));

    info!(url = %app.config.url, "database connection pool created (simulated)");
    Ok(())
}
