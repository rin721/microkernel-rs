use crate::errors::AppError;

/// Port Trait for role-based and attribute-based access control.
///
/// The concrete implementation uses `casbin` internally. Business plugins
/// call `env.rbac().enforce(...)` without any direct dependency on the
/// Casbin crate.
pub trait RbacPort: Send + Sync + 'static {
    /// Evaluate whether `subject` may perform `action` on `object`.
    ///
    /// # Arguments
    /// * `subject` — the acting principal (e.g., user ID or role name)
    /// * `object`  — the resource being accessed (e.g., `/api/orders`)
    /// * `action`  — the operation being attempted (e.g., `"read"`, `"write"`)
    ///
    /// Returns `Ok(true)` when access is granted, `Ok(false)` when denied.
    /// Returns `Err` only if the policy engine itself encounters an internal error.
    fn enforce(
        &self,
        subject: &str,
        object: &str,
        action: &str,
    ) -> impl std::future::Future<Output = Result<bool, AppError>> + Send;

    /// Reload policy rules from the backing store without restarting the component.
    ///
    /// Called when the `RoleUpdated` event is received from the event bus.
    fn reload_policy(&self) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    /// Add a policy rule at runtime.
    fn add_policy(
        &self,
        subject: &str,
        object: &str,
        action: &str,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    /// Remove a policy rule at runtime.
    fn remove_policy(
        &self,
        subject: &str,
        object: &str,
        action: &str,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send;
}
