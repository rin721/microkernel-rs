//! # microkernel-core
//!
//! The microkernel engine. Provides:
//!
//! - [`env`]        — `EnvBuilder` for assembling a concrete `SystemEnv`
//! - [`lifecycle`]  — `Bootstrap` and `Teardown` orchestrators
//! - [`event_bus`]  — `EventDispatcher` bounded broadcast channel

pub mod env;
pub mod event_bus;
pub mod lifecycle;

pub use env::EnvBuilder;
pub use event_bus::EventDispatcher;
pub use lifecycle::{Bootstrap, Teardown};
