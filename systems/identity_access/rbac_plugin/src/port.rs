use microkernel_contracts::{AppError, RbacPort};

pub struct RbacHandle {
    pub(crate) model_path: String,
    pub(crate) policy_path: String,
}

impl RbacPort for RbacHandle {
    async fn enforce(&self, _subject: &str, _object: &str, _action: &str) -> Result<bool, AppError> {
        // 在真实的实现中：
        // let mut enforcer = casbin::Enforcer::new(&self.model_path, &self.policy_path).await.unwrap();
        // Ok(enforcer.enforce((subject, object, action)).unwrap())
        Ok(true)
    }

    async fn add_policy(&self, _subject: &str, _object: &str, _action: &str) -> Result<(), AppError> {
        Ok(())
    }

    async fn remove_policy(&self, _subject: &str, _object: &str, _action: &str) -> Result<(), AppError> {
        Ok(())
    }

    async fn reload_policy(&self) -> Result<(), AppError> {
        Ok(())
    }
}
