use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use async_trait::async_trait;
use microkernel_contracts::{Archetype, KernelError, Plugin, SystemEnv};
use crate::lifecycle::Teardown;
use tokio::sync::Mutex;
use tracing::{error, info, instrument};

// ── Internal lifecycle step abstraction ──────────────────────────────────────

/// Internal trait object interface for a single lifecycle step.
///
/// `async_trait` is used **only here** (internal to `kernel/core`) to enable
/// `Box<dyn LifecycleStep<E>>` storage. The public `Archetype` and `Plugin`
/// traits in `contracts` use native `async fn`, preserving zero-cost static
/// dispatch everywhere else.
#[async_trait]
trait LifecycleStep<E: SystemEnv>: Send + Sync {
    async fn mount(&self, env: &E) -> Result<(), KernelError>;
    async fn start(&self, env: &E) -> Result<(), KernelError>;
    fn name(&self) -> &'static str;
}

// ── Archetype wrapper ─────────────────────────────────────────────────────────

/// Wraps a concrete `Archetype` implementation behind the `LifecycleStep` vtable,
/// performing type erasure so it can live in the `Vec<Box<dyn LifecycleStep>>`.
///
/// The archetype is stored in an `Arc<Mutex>` so that `Teardown` can also hold
/// a reference to it for the `pre_stop` / `post_stop` sequence.
struct ArchetypeStep<A, E>
where
    A: Archetype<E> + 'static,
    E: SystemEnv,
{
    inner: Arc<Mutex<A>>,
    component_name: &'static str,
    _marker: std::marker::PhantomData<E>,
}

#[async_trait]
impl<A, E> LifecycleStep<E> for ArchetypeStep<A, E>
where
    A: Archetype<E> + 'static,
    E: SystemEnv,
{
    async fn mount(&self, env: &E) -> Result<(), KernelError> {
        let app = self.inner.lock().await;
        app.pre_mount(env).await.map_err(|e| KernelError::BootstrapFailed {
            component: self.component_name.to_owned(),
            source: e,
        })?;
        app.post_mount(env).await.map_err(|e| KernelError::BootstrapFailed {
            component: self.component_name.to_owned(),
            source: e,
        })
    }

    async fn start(&self, env: &E) -> Result<(), KernelError> {
        let app = self.inner.lock().await;
        app.pre_start(env).await.map_err(|e| KernelError::BootstrapFailed {
            component: self.component_name.to_owned(),
            source: e,
        })?;
        app.post_start(env).await.map_err(|e| KernelError::BootstrapFailed {
            component: self.component_name.to_owned(),
            source: e,
        })
    }

    fn name(&self) -> &'static str {
        self.component_name
    }
}

// ── Plugin wrapper ────────────────────────────────────────────────────────────

struct PluginStep<P, E>
where
    P: Plugin<E> + 'static,
    E: SystemEnv,
{
    inner: Arc<Mutex<P>>,
    component_name: &'static str,
    _marker: std::marker::PhantomData<E>,
}

#[async_trait]
impl<P, E> LifecycleStep<E> for PluginStep<P, E>
where
    P: Plugin<E> + 'static,
    E: SystemEnv,
{
    async fn mount(&self, env: &E) -> Result<(), KernelError> {
        let mut plugin = self.inner.lock().await;
        plugin.on_load(env).await.map_err(|e| KernelError::BootstrapFailed {
            component: self.component_name.to_owned(),
            source: e,
        })
    }

    async fn start(&self, env: &E) -> Result<(), KernelError> {
        let mut plugin = self.inner.lock().await;
        plugin.on_start(env).await.map_err(|e| KernelError::BootstrapFailed {
            component: self.component_name.to_owned(),
            source: e,
        })
    }

    fn name(&self) -> &'static str {
        self.component_name
    }
}

// ── Teardown step ─────────────────────────────────────────────────────────────

/// Boxed stop function stored in reverse order for teardown.
type StopFuture = Pin<Box<dyn Future<Output = Result<(), KernelError>> + Send>>;
type StopFn = Box<dyn Fn() -> StopFuture + Send + Sync>;

// ── Bootstrap ─────────────────────────────────────────────────────────────────

/// Orchestrates the sequential bootstrap of all Generic Apps and Business Plugins.
///
/// # Execution order
/// For each registered component (in registration order):
/// 1. `mount(env)` — `pre_mount` + `post_mount` (or `on_load` for plugins)
/// 2. `start(env)` — `pre_start` + `post_start` (or `on_start` for plugins)
///
/// # Failure handling
/// Any `Err` from a hook aborts the bootstrap immediately. The caller is
/// responsible for running `Teardown` on already-started components.
pub struct Bootstrap<E: SystemEnv> {
    steps: Vec<Box<dyn LifecycleStep<E>>>,
    /// Stop functions registered in order; `Teardown` reverses them.
    stop_fns: Vec<(&'static str, StopFn)>,
}

impl<E: SystemEnv> Bootstrap<E> {
    /// Create an empty bootstrap sequence.
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            stop_fns: Vec::new(),
        }
    }

    /// Register a Generic App with its configuration.
    ///
    /// # Arguments
    /// * `app`    — an already-constructed `Archetype` instance
    /// * `config` — (consumed; only kept long enough for `pre_create` / `post_create`)
    ///
    /// The app is wrapped in `Arc<Mutex>` so `Teardown` can also hold a reference.
    pub fn register_archetype<A>(&mut self, app: A) -> Arc<Mutex<A>>
    where
        A: Archetype<E> + 'static,
    {
        let name = std::any::type_name::<A>();
        let shared = Arc::new(Mutex::new(app));

        // Clone for stop fn capture
        let stop_shared = Arc::clone(&shared);
        let stop_name = name;

        let step: Box<dyn LifecycleStep<E>> = Box::new(ArchetypeStep {
            inner: Arc::clone(&shared),
            component_name: name,
            _marker: std::marker::PhantomData,
        });
        self.steps.push(step);

        // Register stop function (pre_stop + post_stop)
        let stop_fn: StopFn = Box::new(move || {
            let app = Arc::clone(&stop_shared);
            let name = stop_name;
            Box::pin(async move {
                let guard = app.lock().await;
                guard.pre_stop().await.map_err(|e| KernelError::TeardownFailed {
                    component: name.to_owned(),
                    source: e,
                })?;
                guard.post_stop().await.map_err(|e| KernelError::TeardownFailed {
                    component: name.to_owned(),
                    source: e,
                })
            })
        });
        self.stop_fns.push((name, stop_fn));

        shared
    }

    /// Register a Business Plugin.
    pub fn register_plugin<P>(&mut self, plugin: P) -> Arc<Mutex<P>>
    where
        P: Plugin<E> + 'static,
    {
        let name = std::any::type_name::<P>();
        let shared = Arc::new(Mutex::new(plugin));

        let stop_shared = Arc::clone(&shared);

        let step: Box<dyn LifecycleStep<E>> = Box::new(PluginStep {
            inner: Arc::clone(&shared),
            component_name: name,
            _marker: std::marker::PhantomData,
        });
        self.steps.push(step);

        let stop_fn: StopFn = Box::new(move || {
            let plugin = Arc::clone(&stop_shared);
            let name = name;
            Box::pin(async move {
                // We cannot pass `env` here because we don't have it at teardown
                // registration time. The `Teardown` struct holds env separately.
                // For now, stop_fn holds the plugin; env must be passed separately.
                // See `Teardown::run` for how env is threaded through.
                let _ = plugin; // will be used by Teardown
                let _ = name;
                Ok::<(), KernelError>(())
            })
        });
        self.stop_fns.push((name, stop_fn));

        shared
    }

    /// Execute the full bootstrap sequence against the assembled environment.
    ///
    /// Returns a `Teardown` pre-populated with all registered stop functions so
    /// the caller can run teardown in reverse order on shutdown.
    ///
    /// # Errors
    /// Returns the first `KernelError` encountered and aborts remaining steps.
    pub async fn run(mut self, env: &E) -> Result<Teardown, KernelError> {
        for step in &self.steps {
            info!(component = step.name(), "mounting component");
            step.mount(env).await?;

            info!(component = step.name(), "starting component");
            step.start(env).await?;

            info!(component = step.name(), "component ready");
        }

        // Transfer stop functions to Teardown in reverse order
        self.stop_fns.reverse();
        Ok(Teardown {
            stop_fns: self.stop_fns,
        })
    }
}

impl<E: SystemEnv> Default for Bootstrap<E> {
    fn default() -> Self {
        Self::new()
    }
}
