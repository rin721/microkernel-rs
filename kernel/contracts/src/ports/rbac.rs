use crate::errors::AppError;

/// 针对基于角色和基于属性的访问控制的端口 Trait。
///
/// 具体实现内部使用 `casbin`。业务插件
/// 调用 `env.rbac().enforce(...)`，而没有对
/// Casbin crate 的任何直接依赖。
pub trait RbacPort: Send + Sync + 'static {
    /// 评估 `subject` 是否可以对 `object` 执行 `action`。
    ///
    /// # 参数
    /// * `subject` — 执行操作的主体 (例如，用户 ID 或角色名)
    /// * `object`  — 被访问的资源 (例如，`/api/orders`)
    /// * `action`  — 试图执行的操作 (例如，`"read"`, `"write"`)
    ///
    /// 授权访问时返回 `Ok(true)`，拒绝时返回 `Ok(false)`。
    /// 仅在策略引擎本身遇到内部错误时返回 `Err`。
    fn enforce(
        &self,
        subject: &str,
        object: &str,
        action: &str,
    ) -> impl std::future::Future<Output = Result<bool, AppError>> + Send;

    /// 在不重新启动组件的情况下，从后备存储重新加载策略规则。
    ///
    /// 在从事件总线收到 `RoleUpdated` 事件时调用。
    fn reload_policy(&self) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    /// 在运行时添加策略规则。
    fn add_policy(
        &self,
        subject: &str,
        object: &str,
        action: &str,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    /// 在运行时移除策略规则。
    fn remove_policy(
        &self,
        subject: &str,
        object: &str,
        action: &str,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send;
}
