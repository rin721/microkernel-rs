use std::future::Future;
use std::pin::Pin;

use microkernel_contracts::KernelError;
use tracing::{error, info};

type StopFuture = Pin<Box<dyn Future<Output = Result<(), KernelError>> + Send>>;
type StopFn = Box<dyn Fn() -> StopFuture + Send + Sync>;

/// Orchestrates graceful teardown of all registered components in **reverse**
/// registration order (LIFO — last started, first stopped).
///
/// # Error handling
/// Unlike `Bootstrap` (which aborts on first error), `Teardown` is
/// **non-aborting**: it collects all teardown errors and continues stopping
/// remaining components. All errors are returned at the end.
pub struct Teardown {
    pub(crate) stop_fns: Vec<(&'static str, StopFn)>,
}

impl Teardown {
    /// Execute all registered stop functions in reverse order.
    ///
    /// # Returns
    /// A `Vec` of all errors encountered during teardown. An empty vec means
    /// clean shutdown.
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
