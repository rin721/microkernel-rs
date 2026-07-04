use thiserror::Error;

/// 统一的应用层错误类型。
///
/// 所有通用应用和业务插件**必须**在从任何生命周期钩子返回之前
/// 将其内部错误转换为 `AppError`。这确保了内核可以显示
/// 人类可读的诊断信息，而不依赖于特定于 crate 的类型。
///
/// # 稳定性
/// 此枚举标记为 `#[non_exhaustive]`，以允许在次要版本中添加新变体
/// 而不破坏下游的 `match` 分支（SemVer §5 — 枚举可扩展性）。
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AppError {
    /// 配置字段验证失败（例如，无效 URL，超出范围的值）。
    #[error("configuration error: {0}")]
    Config(String),

    /// 无法初始化资源（例如，连接池创建失败）。
    #[error("initialization error: {0}")]
    Initialization(String),

    /// I/O 操作失败。内部消息被转换为字符串，以避免
    /// 将特定于基础设施的错误类型泄漏到契约边界。
    #[error("i/o error: {0}")]
    Io(String),

    /// 调用者提供了无效或格式错误的输入数据。
    #[error("invalid input: {0}")]
    InvalidInput(String),

    /// 发生意外的内部错误。如果在生产中看到，视为 bug。
    #[error("internal error: {0}")]
    Internal(String),

    /// 组件暂时不可用（例如，连接池耗尽）。
    #[error("service unavailable: {0}")]
    Unavailable(String),

    /// 找不到请求的资源。
    #[error("not found: {0}")]
    NotFound(String),

    /// 调用者缺乏执行请求操作的权限。
    #[error("permission denied: {0}")]
    PermissionDenied(String),
}
