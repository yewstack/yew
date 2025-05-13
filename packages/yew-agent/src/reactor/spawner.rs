use serde::de::Deserialize;
use serde::ser::Serialize;

use super::bridge::ReactorBridge;
use super::scope::ReactorScoped;
use super::traits::Reactor;
use super::worker::ReactorWorker;
use crate::codec::{Bincode, Codec};
use crate::worker::WorkerSpawner;

/// A spawner to create oneshot workers.
#[derive(Debug, Default)]
pub struct ReactorSpawner<R, CODEC = Bincode>
where
    R: Reactor + 'static,
    CODEC: Codec,
{
    inner: WorkerSpawner<ReactorWorker<R>, CODEC>,
}

impl<R, CODEC> ReactorSpawner<R, CODEC>
where
    R: Reactor + 'static,
    CODEC: Codec,
{
    /// Creates a ReactorSpawner.
    pub const fn new() -> Self {
        Self {
            inner: WorkerSpawner::<ReactorWorker<R>, CODEC>::new(),
        }
    }

    /// Sets a new message encoding.
    pub const fn encoding<C>(&self) -> ReactorSpawner<R, C>
    where
        C: Codec,
    {
        ReactorSpawner {
            inner: WorkerSpawner::<ReactorWorker<R>, C>::new(),
        }
    }

    /// Indicates that [`spawn`](WorkerSpawner#method.spawn) should expect a
    /// `path` to a loader shim script (e.g. when using Trunk, created by using
    /// the [`data-loader-shim`](https://trunkrs.dev/assets/#link-asset-types)
    /// asset type) and one does not need to be generated. `false` by default.
    pub fn with_loader(mut self, with_loader: bool) -> Self {
        self.inner.with_loader(with_loader);
        self
    }

    /// Determines whether the worker will be spawned with
    /// [`options.type`](https://developer.mozilla.org/en-US/docs/Web/API/Worker/Worker#type)
    /// set to `module`. `true` by default.
    ///
    /// This option should be un-set if the worker was created with the
    /// `--target no-modules` flag of `wasm-bindgen`. If using Trunk, see the
    /// [`data-bindgen-target`](https://trunkrs.dev/assets/#link-asset-types)
    /// asset type.
    pub fn as_module(mut self, as_module: bool) -> Self {
        self.inner.as_module(as_module);

        self
    }

    /// Spawns a reactor worker.
    pub fn spawn(mut self, path: &str) -> ReactorBridge<R>
    where
        <R::Scope as ReactorScoped>::Input: Serialize + for<'de> Deserialize<'de>,
        <R::Scope as ReactorScoped>::Output: Serialize + for<'de> Deserialize<'de>,
    {
        let rx = ReactorBridge::register_callback(&mut self.inner);

        let inner = self.inner.spawn(path);

        ReactorBridge::new(inner, rx)
    }
}
