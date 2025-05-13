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

    /// Spawns an Oneshot Worker.
    pub fn spawn(mut self, path: &str) -> OneshotBridge<N>
    where
        N::Input: Serialize + for<'de> Deserialize<'de>,
        N::Output: Serialize + for<'de> Deserialize<'de>,
    {
        let rx = OneshotBridge::register_callback(&mut self.inner);

        let inner = self.inner.spawn(path);

        OneshotBridge::new(inner, rx)
    }

    /// Spawns an Oneshot Worker with a loader shim script.
    pub fn spawn_with_loader(mut self, loader_path: &str) -> OneshotBridge<N>
    where
        N::Input: Serialize + for<'de> Deserialize<'de>,
        N::Output: Serialize + for<'de> Deserialize<'de>,
    {
        let rx = OneshotBridge::register_callback(&mut self.inner);

        let inner = self.inner.spawn_with_loader(loader_path);

        OneshotBridge::new(inner, rx)
    }
}
