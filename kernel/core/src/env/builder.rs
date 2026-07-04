use microkernel_contracts::{KernelError, SystemEnv};

/// A type-safe builder for assembling a concrete `SystemEnv` implementation.
///
/// `EnvBuilder` acts as a staging area: callers supply each infrastructure
/// component in turn, and `build()` finalises the environment. The generic
/// parameter `E` is fixed at construction time; every setter is a no-op type
/// check that ensures the supplied component matches the associated type declared
/// in `E`.
///
/// # Usage
/// ```rust,ignore
/// let env = EnvBuilder::<ProdEnv>::new()
///     .build(prod_env_instance)?;
/// ```
///
/// For the current stage of the project the builder is a thin wrapper that
/// accepts a fully-constructed `E` directly. Future iterations may grow
/// individual setters for incremental construction and validation.
pub struct EnvBuilder<E: SystemEnv> {
    /// Intermediate assembled environment, set once `set_env` is called.
    inner: Option<E>,
}

impl<E: SystemEnv> EnvBuilder<E> {
    /// Create a new, empty builder.
    pub fn new() -> Self {
        Self { inner: None }
    }

    /// Supply the fully-assembled environment value.
    ///
    /// This replaces any previously supplied value. Typically called once from
    /// `host/src/main.rs` after constructing all Generic App instances.
    #[must_use]
    pub fn set_env(mut self, env: E) -> Self {
        self.inner = Some(env);
        self
    }

    /// Finalise the builder and return the assembled environment.
    ///
    /// # Errors
    /// Returns `KernelError::EnvBuildFailed` if `set_env` was never called.
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
