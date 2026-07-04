#![allow(async_fn_in_trait)]
//! # microkernel-contracts
//!
//! Zero-dependency trait abstractions for the microkernel system.
//!
//! This crate is the **Single Source of Truth** for all cross-crate contracts.
//! It must remain free of any business logic or infrastructure-specific types.
//!
//! ## Module layout
//!
//! - [`errors`]   — `AppError` and `KernelError` definitions
//! - [`ports`]    — Port Trait definitions for each infrastructure capability
//! - [`env`]      — `SystemEnv` — the global static environment constraint
//! - [`lifecycle`] — `Archetype` and `Plugin` lifecycle hook traits

pub mod errors;
pub mod env;
pub mod lifecycle;
pub mod ports;

// ── Top-level re-exports for convenience ─────────────────────────────────────
pub use errors::{AppError, KernelError};
pub use env::SystemEnv;
pub use lifecycle::{Archetype, HealthStatus, Plugin};
pub use ports::{AuthPort, CachePort, DatabasePort, LoggerPort, RbacPort, StoragePort};
