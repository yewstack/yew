use super::*;
use crate::callback::Callback;
use queue::Queue;
use std::fmt;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicUsize, Ordering};
use web_sys::Worker;

thread_local! {
    static QUEUE: Queue<usize> = Queue::new();
}

static PRIVATE_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
const SINGLETON_ID: HandlerId = HandlerId(0, true);

/// Create a new instance for every bridge.
#[allow(missing_debug_implementations)]
pub struct Private<AGN> {
    _agent: PhantomData<AGN>,
}

impl<AGN> Discoverer for Private<AGN>
where
    AGN: Agent,
    <AGN as Agent>::Input: Serialize + for<'de> Deserialize<'de>,
    <AGN as Agent>::Output: Serialize + for<'de> Deserialize<'de>,
{
    type Agent = AGN;

    fn spawn_or_join(callback: Option<Callback<AGN::Output>>) -> Box<dyn Bridge<AGN>> {
        let id = PRIVATE_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        let callback = callback.expect("Callback required for Private agents");
        let handler = move |data: Vec<u8>, worker: &Worker| {
            let msg = FromWorker::<AGN::Output>::unpack(&data);
            match msg {
                FromWorker::WorkerLoaded => {
                    QUEUE.with(|queue| {
                        queue.insert_loaded_agent(id);

                        if let Some(msgs) = queue.remove_msg_queue(&id) {
                            for msg in msgs {
                                worker.post_message_vec(msg)
                            }
                        }
                    });
                }
                FromWorker::ProcessOutput(id, output) => {
                    assert_eq!(id.raw_id(), SINGLETON_ID.raw_id());
                    callback.emit(output);
                }
            }
        };

        // TODO(#947): Drop handler when bridge is dropped
        let name_of_resource = AGN::name_of_resource();
        let worker = {
            let worker = worker_new(name_of_resource, AGN::is_module());
            let worker_clone = worker.clone();
            worker.set_onmessage_closure(move |data: Vec<u8>| handler(data, &worker_clone));
            worker
        };
        let bridge = PrivateBridge {
            worker,
            _agent: PhantomData,
            id,
        };
        bridge.send_message(ToWorker::Connected(SINGLETON_ID));
        Box::new(bridge)
    }
}

/// A connection manager for components interaction with workers.
pub struct PrivateBridge<AGN>
where
    AGN: Agent,
    <AGN as Agent>::Input: Serialize + for<'de> Deserialize<'de>,
    <AGN as Agent>::Output: Serialize + for<'de> Deserialize<'de>,
{
    worker: Worker,
    _agent: PhantomData<AGN>,
    id: usize,
}

impl<AGN> PrivateBridge<AGN>
where
    AGN: Agent,
    <AGN as Agent>::Input: Serialize + for<'de> Deserialize<'de>,
    <AGN as Agent>::Output: Serialize + for<'de> Deserialize<'de>,
{
    /// Send a message to the worker, queuing the message if necessary
    fn send_message(&self, msg: ToWorker<AGN::Input>) {
        QUEUE.with(|queue| {
            if queue.is_worker_loaded(&self.id) {
                send_to_remote::<AGN>(&self.worker, msg);
            } else {
                queue.add_msg_to_queue(msg.pack(), self.id);
            }
        });
    }
}
impl<AGN> fmt::Debug for PrivateBridge<AGN>
where
    AGN: Agent,
    <AGN as Agent>::Input: Serialize + for<'de> Deserialize<'de>,
    <AGN as Agent>::Output: Serialize + for<'de> Deserialize<'de>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PrivateBridge<_>")
    }
}

impl<AGN> Bridge<AGN> for PrivateBridge<AGN>
where
    AGN: Agent,
    <AGN as Agent>::Input: Serialize + for<'de> Deserialize<'de>,
    <AGN as Agent>::Output: Serialize + for<'de> Deserialize<'de>,
{
    fn send(&mut self, msg: AGN::Input) {
        let msg = ToWorker::ProcessInput(SINGLETON_ID, msg);
        self.send_message(msg);
    }
}

impl<AGN> Drop for PrivateBridge<AGN>
where
    AGN: Agent,
    <AGN as Agent>::Input: Serialize + for<'de> Deserialize<'de>,
    <AGN as Agent>::Output: Serialize + for<'de> Deserialize<'de>,
{
    fn drop(&mut self) {
        let disconnected = ToWorker::Disconnected(SINGLETON_ID);
        send_to_remote::<AGN>(&self.worker, disconnected);

        let destroy = ToWorker::Destroy;
        send_to_remote::<AGN>(&self.worker, destroy);

        QUEUE.with(|queue| {
            queue.remove_agent(&self.id);
        });
    }
}
