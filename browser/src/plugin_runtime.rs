use std::future::Future;
use std::io;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use tokio::runtime::Runtime;

/// A Tokio runtime owned by the plugin.
///
/// A cdylib compiles its own Tokio, so the host runtime is invisible to it.
/// Tools that spawn tasks or drive I/O must be built and polled with this
/// runtime entered.
#[derive(Clone, Debug)]
pub(super) struct PluginRuntime(Arc<Runtime>);

impl PluginRuntime {
    pub(super) fn new() -> io::Result<Self> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;
        Ok(Self(Arc::new(runtime)))
    }

    pub(super) fn build<T>(&self, build: impl FnOnce() -> T) -> Option<T> {
        let _guard = self.0.enter();
        catch_unwind(AssertUnwindSafe(build)).ok()
    }

    pub(super) fn wrap<F>(&self, future: F) -> InRuntime<F> {
        InRuntime {
            runtime: Arc::clone(&self.0),
            future,
        }
    }
}

/// Polls a future with the plugin runtime entered for each poll.
pub(super) struct InRuntime<F> {
    runtime: Arc<Runtime>,
    future: F,
}

impl<F> Future for InRuntime<F>
where
    F: Future + Unpin,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let _guard = this.runtime.enter();
        Pin::new(&mut this.future).poll(context)
    }
}

#[cfg(test)]
#[path = "plugin_runtime_tests.rs"]
mod tests;
