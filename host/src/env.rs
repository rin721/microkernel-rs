use std::sync::Arc;
use microkernel_contracts::{DatabasePort, CachePort, StoragePort, AuthPort, RbacPort, LoggerPort, SystemEnv};

use microkernel_logger::LoggerHandle;
use microkernel_database::DbHandle;
use microkernel_cache::CacheHandle;
use microkernel_storage::StorageHandle;
use microkernel_auth::AuthHandle;
use microkernel_rbac::RbacHandle;

/// 主机应用的具体 `SystemEnv` 实现。
#[derive(Clone)]
pub struct ProdEnv {
    logger: Arc<LoggerHandle>,
    db: Arc<DbHandle>,
    cache: Arc<CacheHandle>,
    storage: Arc<StorageHandle>,
    auth: Arc<AuthHandle>,
    rbac: Arc<RbacHandle>,
}

impl ProdEnv {
    pub fn new(
        logger: Arc<LoggerHandle>,
        db: Arc<DbHandle>,
        cache: Arc<CacheHandle>,
        storage: Arc<StorageHandle>,
        auth: Arc<AuthHandle>,
        rbac: Arc<RbacHandle>,
    ) -> Self {
        Self {
            logger,
            db,
            cache,
            storage,
            auth,
            rbac,
        }
    }
}

impl SystemEnv for ProdEnv {
    type Db = DbHandle;
    type Cache = CacheHandle;
    type Storage = StorageHandle;
    type Auth = AuthHandle;
    type Rbac = RbacHandle;
    type Logger = LoggerHandle;

    fn db(&self) -> &Self::Db {
        &self.db
    }

    fn cache(&self) -> &Self::Cache {
        &self.cache
    }

    fn storage(&self) -> &Self::Storage {
        &self.storage
    }

    fn auth(&self) -> &Self::Auth {
        &self.auth
    }

    fn rbac(&self) -> &Self::Rbac {
        &self.rbac
    }

    fn logger(&self) -> &Self::Logger {
        &self.logger
    }
}
