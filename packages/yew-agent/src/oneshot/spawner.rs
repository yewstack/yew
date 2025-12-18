use serde::de::Deserialize;
use serde::ser::Serialize;

use super::bridge::OneshotBridge;
use super::traits::Oneshot;
use super::worker::OneshotWorker;
use crate::codec::{Bincode, Codec};
use crate::worker::WorkerSpawner;

/// A spawner to create oneshot workers.
#[derive(Debug, Default)]
pub struct OneshotSpawner<N, CODEC = Bincode>
where
    N: Oneshot + 'static,
    CODEC: Codec,
{
    inner: WorkerSpawner<OneshotWorker<N>, CODEC>,
}

impl<N, CODEC> OneshotSpawner<N, CODEC>
where
    N: Oneshot + 'static,
    CODEC: Codec,
{
    /// Creates a [OneshotSpawner].
    pub const fn new() -> Self {
        Self {
            inner: WorkerSpawner::<OneshotWorker<N>, CODEC>::new(),
        }
    }

    /// Sets a new message encoding.
    pub const fn encoding<C>(&self) -> OneshotSpawner<N, C>
    where
        C: Codec,
    {
        OneshotSpawner {
            inner: WorkerSpawner::<OneshotWorker<N>, C>::new(),
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

    /// Spawns a Oneshot Worker.
    pub fn spawn(mut self, path: &str) -> OneshotBridge<N>
    where
        N::Input: Serialize + for<'de> Deserialize<'de>,
        N::Output: Serialize + for<'de> Deserialize<'de>,
    {
        let rx = OneshotBridge::register_callback(&mut self.inner);

        let inner = self.inner.spawn(path);

        OneshotBridge::new(inner, rx)
    }
}
