use std::fmt;
use std::marker::PhantomData;

use serde::de::Deserialize;
use serde::ser::Serialize;

use super::lifecycle::WorkerLifecycleEvent;
use super::messages::{FromWorker, ToWorker};
use super::native_worker::{DedicatedWorker, NativeWorkerExt, WorkerSelf};
use super::scope::WorkerScope;
use super::traits::Worker;
use crate::codec::{Bincode, Codec};

/// A Worker Registrar.
pub struct WorkerRegistrar<W, CODEC = Bincode>
where
    W: Worker,
    CODEC: Codec,
{
    _marker: PhantomData<(W, CODEC)>,
}

impl<W: Worker> fmt::Debug for WorkerRegistrar<W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("WorkerRegistrar<_>")
    }
}

impl<W, CODEC> WorkerRegistrar<W, CODEC>
where
    W: Worker + 'static,
    CODEC: Codec,
{
    pub(crate) fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    /// Sets a new message encoding.
    pub fn encoding<C>(&self) -> WorkerRegistrar<W, C>
    where
        C: Codec,
    {
        WorkerRegistrar::new()
    }

    /// Executes an worker in the current environment.
    pub fn register(&self)
    where
        CODEC: Codec,
        W::Input: Serialize + for<'de> Deserialize<'de>,
        W::Output: Serialize + for<'de> Deserialize<'de>,
    {
        let scope = WorkerScope::<W>::new::<CODEC>();
        let upd = WorkerLifecycleEvent::Create(scope.clone());
        scope.send(upd);
        let handler = move |msg: ToWorker<W>| {
            let upd = WorkerLifecycleEvent::Remote(msg);
            scope.send(upd);
        };
        let loaded: FromWorker<W> = FromWorker::WorkerLoaded;
        let worker = DedicatedWorker::worker_self();
        worker.set_on_packed_message::<_, CODEC, _>(handler);
        worker.post_packed_message::<_, CODEC>(loaded);
    }
}
