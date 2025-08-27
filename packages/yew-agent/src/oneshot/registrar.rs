use std::fmt;

use serde::de::Deserialize;
use serde::ser::Serialize;

use super::traits::Oneshot;
use super::worker::OneshotWorker;
use crate::codec::{Bincode, Codec};
use crate::traits::Registrable;
use crate::worker::WorkerRegistrar;

/// A registrar for oneshot workers.
pub struct OneshotRegistrar<T, CODEC = Bincode>
where
    T: Oneshot + 'static,
    CODEC: Codec + 'static,
{
    inner: WorkerRegistrar<OneshotWorker<T>, CODEC>,
}

impl<T, CODEC> Default for OneshotRegistrar<T, CODEC>
where
    T: Oneshot + 'static,
    CODEC: Codec + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<N, CODEC> OneshotRegistrar<N, CODEC>
where
    N: Oneshot + 'static,
    CODEC: Codec + 'static,
{
    /// Creates a new Oneshot Registrar.
    pub fn new() -> Self {
        Self {
            inner: OneshotWorker::<N>::registrar().encoding::<CODEC>(),
        }
    }

    /// Sets the encoding.
    pub fn encoding<C>(&self) -> OneshotRegistrar<N, C>
    where
        C: Codec + 'static,
    {
        OneshotRegistrar {
            inner: self.inner.encoding::<C>(),
        }
    }

    /// Registers the worker.
    pub fn register(&self)
    where
        N::Input: Serialize + for<'de> Deserialize<'de>,
        N::Output: Serialize + for<'de> Deserialize<'de>,
    {
        self.inner.register()
    }
}

impl<T, CODEC> fmt::Debug for OneshotRegistrar<T, CODEC>
where
    T: Oneshot + 'static,
    CODEC: Codec + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OneshotRegistrar<_>").finish()
    }
}
