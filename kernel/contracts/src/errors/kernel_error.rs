use thiserror::Error;

use crate::errors::AppError;

/// 引导和销毁失败的内核级错误类型。
///
/// 与 `AppError`（从应用程序代码中产生）不同，当生命周期编排发生严重失败时，
/// 微内核引擎本身会引发 `KernelError`。
///
/// # 稳定性
/// `#[non_exhaustive]` — 参见 `AppError` 文档中的基本原理。
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum KernelError {
    /// 组件在引导（Load → Mount → Start）阶段失败。
    #[error("bootstrap failed for component '{component}': {source}")]
    BootstrapFailed {
        /// 失败组件的人类可读名称（例如，`"DatabaseApp"`）。
        component: String,
        /// 导致失败的底层应用程序错误。
        source: AppError,
    },

    /// 组件在销毁（PreStop → PostStop）阶段失败。
    ///
    /// 销毁错误是**非致命的** —— 内核记录它们并继续
    /// 停止剩余的组件。所有的销毁错误都会在最后
    /// 收集并一起报告。
    #[error("teardown failed for component '{component}': {source}")]
    TeardownFailed {
        component: String,
        source: AppError,
    },

    /// 环境无法构建（例如，缺少必需的端口）。
    #[error("environment build failed: {0}")]
    EnvBuildFailed(String),

    /// 事件总线通道已达到容量上限；发布者必须退避。
    ///
    /// 当有界通道缓冲区已满时，`EventDispatcher::publish` 会返回此错误，
    /// 以向调用方明确显示背压。
    /// **绝不能被静默丢弃。**
    #[error("event bus backpressure exceeded: channel capacity reached")]
    BackpressureExceeded,

    /// 无法传递任务取消信号。
    #[error("cancellation signal failed: {0}")]
    CancellationFailed(String),
}
