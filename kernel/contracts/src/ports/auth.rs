use std::collections::HashMap;

use crate::errors::AppError;

/// 解码后的 JWT 声明，作为简单的键值映射呈现。
///
/// 使用 `HashMap<String, String>` 避免将 `jsonwebtoken` 或 `paseto`
/// 类型泄漏到契约边界，同时仍然提供对所有
/// 自定义声明的完全访问权限。
#[derive(Debug, Clone)]
pub struct Claims {
    /// 主题 (例如，用户 ID) — 对应 JWT 的 `sub` 字段。
    pub subject: String,
    /// Unix 时间戳格式的令牌过期时间。
    pub expires_at: i64,
    /// 从令牌有效负载解码出的所有附加自定义声明。
    pub extra: HashMap<String, String>,
}

/// 基于 JWT 的身份验证的端口 Trait。
///
/// 仅涵盖令牌的签发和验证。会话存储和用户查找
/// 是高级业务插件的责任。
pub trait AuthPort: Send + Sync + 'static {
    /// 为给定主体签发一个签名的 JWT，嵌入任何额外的声明。
    ///
    /// # 参数
    /// * `subject`    — 主体的唯一标识符 (例如，用户 UUID)
    /// * `extra`      — 要嵌入有效负载中的其他声明
    ///
    /// 返回紧凑序列化的令牌字符串。
    fn sign(
        &self,
        subject: &str,
        extra: HashMap<String, String>,
    ) -> impl std::future::Future<Output = Result<String, AppError>> + Send;

    /// 验证令牌字符串并解码其声明。
    ///
    /// 如果令牌已过期、格式错误，或者签名不匹配，返回 `AppError::PermissionDenied`。
    fn verify(
        &self,
        token: &str,
    ) -> impl std::future::Future<Output = Result<Claims, AppError>> + Send;
}
