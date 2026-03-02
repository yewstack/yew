use std::fmt;

use serde::de::Deserialize;
use serde::ser::Serialize;

use super::scope::ReactorScoped;
use super::traits::Reactor;
use super::worker::ReactorWorker;
use crate::codec::{Bincode, Codec};
use crate::traits::Registrable;
use crate::worker::WorkerRegistrar;

/// A registrar for reactor workers.
pub struct ReactorRegistrar<R, CODEC = Bincode>
where
    R: Reactor + 'static,
    CODEC: Codec + 'static,
{
    inner: WorkerRegistrar<ReactorWorker<R>, CODEC>,
}

impl<R, CODEC> Default for ReactorRegistrar<R, CODEC>
where
    R: Reactor + 'static,
    CODEC: Codec + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<R, CODEC> ReactorRegistrar<R, CODEC>
where
    R: Reactor + 'static,
    CODEC: Codec + 'static,
{
    /// Creates a new reactor registrar.
    pub fn new() -> Self {
        Self {
            inner: ReactorWorker::<R>::registrar().encoding::<CODEC>(),
        }
    }

    /// Sets the encoding.
    pub fn encoding<C>(&self) -> ReactorRegistrar<R, C>
    where
        C: Codec + 'static,
    {
        ReactorRegistrar {
            inner: self.inner.encoding::<C>(),
        }
    }

    /// Registers the worker.
    pub fn register(&self)
    where
        <R::Scope as ReactorScoped>::Input: Serialize + for<'de> Deserialize<'de>,
        <R::Scope as ReactorScoped>::Output: Serialize + for<'de> Deserialize<'de>,
    {
        self.inner.register()
    }
}

impl<R, CODEC> fmt::Debug for ReactorRegistrar<R, CODEC>
where
    R: Reactor + 'static,
    CODEC: Codec + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ReactorRegistrar<_>").finish()
    }
}
