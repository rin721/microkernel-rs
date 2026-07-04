use microkernel_contracts::{KernelError, SystemEnv};

/// 用于组装具体 `SystemEnv` 实现的类型安全构建器。
///
/// `EnvBuilder` 充当暂存区：调用者依次提供每个基础设施组件，
/// 然后 `build()` 最终确定环境。泛型参数 `E` 在构建时固定；
/// 每个 setter 都是一个无操作的类型检查，确保提供的组件与 `E` 中声明的关联类型匹配。
///
/// # Usage
/// ```rust,ignore
/// let env = EnvBuilder::<ProdEnv>::new()
///     .build(prod_env_instance)?;
/// ```
///
/// 在项目当前阶段，构建器是一个薄包装器，直接接受完全构建的 `E`。
/// 未来的迭代可能会增加单独的 setter 用于增量构建和验证。
pub struct EnvBuilder<E: SystemEnv> {
    /// 中间组装的环境，在调用 `set_env` 后设置。
    inner: Option<E>,
}

impl<E: SystemEnv> EnvBuilder<E> {
    /// 创建一个新的、空的构建器。
    pub fn new() -> Self {
        Self { inner: None }
    }

    /// 提供完全组装的环境值。
    ///
    /// 这会替换之前提供的任何值。通常在构建完所有泛型应用实例后，
    /// 从 `host/src/main.rs` 调用一次。
    #[must_use]
    pub fn set_env(mut self, env: E) -> Self {
        self.inner = Some(env);
        self
    }

    /// 最终确定构建器并返回组装的环境。
    ///
    /// # Errors
    /// 如果从未调用 `set_env`，则返回 `KernelError::EnvBuildFailed`。
    pub fn build(self) -> Result<E, KernelError> {
        self.inner.ok_or_else(|| {
            KernelError::EnvBuildFailed(
                "no environment value was provided via `set_env`".to_owned(),
            )
        })
    }
}

impl<E: SystemEnv> Default for EnvBuilder<E> {
    fn default() -> Self {
        Self::new()
    }
}
