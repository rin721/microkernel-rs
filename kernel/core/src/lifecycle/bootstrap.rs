use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use async_trait::async_trait;
use microkernel_contracts::{Archetype, KernelError, Plugin, SystemEnv};
use crate::lifecycle::Teardown;
use tokio::sync::Mutex;
use tracing::{error, info, instrument};

// ── 内部生命周期步骤抽象 ──────────────────────────────────────

/// 单个生命周期步骤的内部 trait 对象接口。
///
/// **仅在此处**（`kernel/core` 内部）使用 `async_trait`，以启用
/// `Box<dyn LifecycleStep<E>>` 存储。`contracts` 中公共的 `Archetype` 和 `Plugin`
/// traits 使用原生的 `async fn`，在其他所有地方保留零成本静态分发。
#[async_trait]
trait LifecycleStep<E: SystemEnv>: Send + Sync {
    async fn mount(&self, env: &E) -> Result<(), KernelError>;
    async fn start(&self, env: &E) -> Result<(), KernelError>;
    fn name(&self) -> &'static str;
}

// ── Archetype 包装器 ─────────────────────────────────────────────────────────

/// 将具体的 `Archetype` 实现包装在 `LifecycleStep` 虚函数表后面，
/// 执行类型擦除，以便它可以存在于 `Vec<Box<dyn LifecycleStep>>` 中。
///
/// archetype 存储在 `Arc<Mutex>` 中，以便 `Teardown` 也可以持有
/// 对它的引用，用于 `pre_stop` / `post_stop` 序列。
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

// ── Plugin 包装器 ────────────────────────────────────────────────────────────

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

// ── 拆卸步骤 ─────────────────────────────────────────────────────────────

/// 为拆卸而以相反顺序存储的装箱停止函数。
type StopFuture = Pin<Box<dyn Future<Output = Result<(), KernelError>> + Send>>;
type StopFn = Box<dyn Fn() -> StopFuture + Send + Sync>;

// ── Bootstrap 引导 ─────────────────────────────────────────────────────────────────

/// 协调所有泛型应用和业务插件的顺序引导。
///
/// # Execution order
/// 对于每个注册的组件（按注册顺序）：
/// 1. `mount(env)` — `pre_mount` + `post_mount` (或插件的 `on_load`)
/// 2. `start(env)` — `pre_start` + `post_start` (或插件的 `on_start`)
///
/// # Failure handling
/// 钩子返回的任何 `Err` 都会立即中止引导。调用者有责任
/// 对已启动的组件运行 `Teardown`。
pub struct Bootstrap<E: SystemEnv> {
    steps: Vec<Box<dyn LifecycleStep<E>>>,
    /// 按顺序注册的停止函数；`Teardown` 会颠倒它们。
    stop_fns: Vec<(&'static str, StopFn)>,
}

impl<E: SystemEnv> Bootstrap<E> {
    /// 创建一个空的引导序列。
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            stop_fns: Vec::new(),
        }
    }

    /// 使用其配置注册一个泛型应用。
    ///
    /// # Arguments
    /// * `app`    — 一个已构建的 `Archetype` 实例
    /// * `config` — （被消耗；只保留足够长的时间用于 `pre_create` / `post_create`）
    ///
    /// 应用被包装在 `Arc<Mutex>` 中，因此 `Teardown` 也可以持有引用。
    pub fn register_archetype<A>(&mut self, app: A) -> Arc<Mutex<A>>
    where
        A: Archetype<E> + 'static,
    {
        let name = std::any::type_name::<A>();
        let shared = Arc::new(Mutex::new(app));

        // 克隆以捕获停止函数
        let stop_shared = Arc::clone(&shared);
        let stop_name = name;

        let step: Box<dyn LifecycleStep<E>> = Box::new(ArchetypeStep {
            inner: Arc::clone(&shared),
            component_name: name,
            _marker: std::marker::PhantomData,
        });
        self.steps.push(step);

        // 注册停止函数 (pre_stop + post_stop)
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

    /// 注册一个业务插件。
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
                // 我们不能在这里传递 `env`，因为在拆卸注册时我们没有它。
                // `Teardown` 结构体单独持有 env。
                // 目前，stop_fn 持有插件；env 必须单独传递。
                // 参见 `Teardown::run` 了解 env 是如何穿透的。
                let _ = plugin; // 将由 Teardown 使用
                let _ = name;
                Ok::<(), KernelError>(())
            })
        });
        self.stop_fns.push((name, stop_fn));

        shared
    }

    /// 针对组装好的环境执行完整的引导序列。
    ///
    /// 返回一个预先填充了所有注册的停止函数的 `Teardown`，
    /// 以便调用者可以在关闭时以相反顺序运行拆卸。
    ///
    /// # Errors
    /// 返回遇到的第一个 `KernelError` 并中止剩余步骤。
    pub async fn run(mut self, env: &E) -> Result<Teardown, KernelError> {
        for step in &self.steps {
            info!(component = step.name(), "mounting component");
            step.mount(env).await?;

            info!(component = step.name(), "starting component");
            step.start(env).await?;

            info!(component = step.name(), "component ready");
        }

        // 将停止函数以相反顺序转移到 Teardown
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
