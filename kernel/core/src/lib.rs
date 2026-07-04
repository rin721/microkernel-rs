//! # microkernel-core
//!
//! 微内核引擎。提供：
//!
//! - [`env`]        — 用于组装具体 `SystemEnv` 的 `EnvBuilder`
//! - [`lifecycle`]  — `Bootstrap` 和 `Teardown` 编排器
//! - [`event_bus`]  — `EventDispatcher` 有界广播通道

pub mod env;
pub mod event_bus;
pub mod lifecycle;

pub use env::EnvBuilder;
pub use event_bus::EventDispatcher;
pub use lifecycle::{Bootstrap, Teardown};
