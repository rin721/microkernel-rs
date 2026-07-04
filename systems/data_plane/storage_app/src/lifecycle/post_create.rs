use microkernel_contracts::AppError;
use tracing::info;
use crate::StorageApp;
use crate::provider::{local_impl, s3_impl};

pub async fn run(app: &StorageApp) -> Result<(), AppError> {
    use crate::config::StorageBackend;

    let op = match app.config.backend {
        StorageBackend::Local => local_impl::build_operator(&app.config)?,
        StorageBackend::S3 => s3_impl::build_operator(&app.config)?,
        StorageBackend::Oss => s3_impl::build_operator(&app.config)?, // 暂时使用 S3 构建器模拟 OSS
    };

    info!(backend = ?app.config.backend, "storage operator initialized");
    Ok(())
}
