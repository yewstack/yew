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

    /// Spawns a Reactor Worker with a loader shim script.
    pub fn spawn_with_loader(mut self, loader_path: &str) -> ReactorBridge<R>
    where
        <R::Scope as ReactorScoped>::Input: Serialize + for<'de> Deserialize<'de>,
        <R::Scope as ReactorScoped>::Output: Serialize + for<'de> Deserialize<'de>,
    {
        let rx = ReactorBridge::register_callback(&mut self.inner);

        let inner = self.inner.spawn_with_loader(loader_path);

        ReactorBridge::new(inner, rx)
    }
}
