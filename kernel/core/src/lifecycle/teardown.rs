use std::future::Future;
use std::pin::Pin;

use microkernel_contracts::KernelError;
use tracing::{error, info};

type StopFuture = Pin<Box<dyn Future<Output = Result<(), KernelError>> + Send>>;
type StopFn = Box<dyn Fn() -> StopFuture + Send + Sync>;

/// 以**相反**的注册顺序协调所有已注册组件的优雅拆卸
///（LIFO — 最后启动，最先停止）。
///
/// # Error handling
/// 与 `Bootstrap`（在第一个错误时中止）不同，`Teardown` 是
/// **非中止的**：它收集所有拆卸错误并继续停止
/// 剩余组件。所有错误将在最后返回。
pub struct Teardown {
    pub(crate) stop_fns: Vec<(&'static str, StopFn)>,
}

impl Teardown {
    /// 以相反顺序执行所有注册的停止函数。
    ///
    /// # Returns
    /// 包含拆卸期间遇到的所有错误的 `Vec`。空的 vec 表示
    /// 干净的关闭。
    pub async fn run(self) -> Vec<KernelError> {
        let mut errors = Vec::new();

        for (name, stop_fn) in self.stop_fns {
            info!(component = name, "stopping component");
            match stop_fn().await {
                Ok(()) => {
                    info!(component = name, "component stopped cleanly");
                }
                Err(e) => {
                    error!(component = name, error = %e, "teardown error — continuing");
                    errors.push(e);
                }
            }
        }

        errors
    }
}
