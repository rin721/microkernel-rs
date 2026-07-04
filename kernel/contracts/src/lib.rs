#![allow(async_fn_in_trait)]
//! # microkernel-contracts
//!
//! 微内核系统的零依赖 trait 抽象。
//!
//! 这个 crate 是所有跨 crate 契约的**唯一真实来源**。
//! 它必须不包含任何业务逻辑或特定于基础设施的类型。
//!
//! ## 模块布局
//!
//! - [`errors`]   — `AppError` 和 `KernelError` 定义
//! - [`ports`]    — 每个基础架构功能的端口 Trait 定义
//! - [`env`]      — `SystemEnv` — 全局静态环境约束
//! - [`lifecycle`] — `Archetype` 和 `Plugin` 生命周期钩子 trait

pub mod errors;
pub mod env;
pub mod lifecycle;
pub mod ports;

// ── 顶层重新导出以方便使用 ─────────────────────────────────────
pub use errors::{AppError, KernelError};
pub use env::SystemEnv;
pub use lifecycle::{Archetype, HealthStatus, Plugin};
pub use ports::{AuthPort, CachePort, DatabasePort, LoggerPort, RbacPort, StoragePort};
