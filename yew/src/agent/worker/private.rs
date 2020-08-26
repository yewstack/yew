use super::*;
use crate::callback::Callback;
use cfg_if::cfg_if;
use cfg_match::cfg_match;
use queue::Queue;
use std::any::TypeId;
use std::fmt;
use std::marker::PhantomData;
cfg_if! {
    if #[cfg(feature = "std_web")] {
        use stdweb::Value;
        #[allow(unused_imports)]
        use stdweb::{_js_impl, js};
    } else if #[cfg(feature = "web_sys")] {
        use web_sys::{Worker};
    }
}

thread_local! {
    static QUEUE: Queue = Queue::new();
}

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
        let callback = callback.expect("Callback required for Private agents");
        let handler = move |data: Vec<u8>,
                            #[cfg(feature = "std_web")] worker: Value,
                            #[cfg(feature = "web_sys")] worker: &Worker| {
            let msg = FromWorker::<AGN::Output>::unpack(&data);
            match msg {
                FromWorker::WorkerLoaded => {
                    QUEUE.with(|queue| {
                        queue.insert_loaded_agent(TypeId::of::<AGN>());

                        let mut msg_queue = queue.borrow_msg_queue_mut();
                        if let Some(msgs) = msg_queue.get_mut(&TypeId::of::<AGN>()) {
                            for msg in msgs.drain(..) {
                                cfg_match! {
                                    feature = "std_web" => ({
                                        let worker = &worker;
                                        js! {@{worker}.postMessage(@{msg});};
                                    }),
                                    feature = "web_sys" => worker.post_message_vec(msg),
                                }
                            }
                        }
                    });
                    send_to_remote::<AGN>(&worker, ToWorker::Connected(SINGLETON_ID));
                }
                FromWorker::ProcessOutput(id, output) => {
                    assert_eq!(id.raw_id(), SINGLETON_ID.raw_id());
                    callback.emit(output);
                }
            }
        };

        // TODO(#947): Drop handler when bridge is dropped
        let name_of_resource = AGN::name_of_resource();
        let worker = cfg_match! {
            feature = "std_web" => js! {
                var worker = new Worker(@{name_of_resource});
                var handler = @{handler};
                worker.onmessage = function(event) {
                    handler(event.data, worker);
                };
                return worker;
            },
            feature = "web_sys" => ({
                let worker = worker_new(name_of_resource, AGN::is_module());
                let worker_clone = worker.clone();
                worker.set_onmessage_closure(move |data: Vec<u8>| handler(data, &worker_clone));
                worker
            }),
        };
        let bridge = PrivateBridge {
            worker,
            _agent: PhantomData,
        };
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
    #[cfg(feature = "std_web")]
    worker: Value,
    #[cfg(feature = "web_sys")]
    worker: Worker,
    _agent: PhantomData<AGN>,
}

impl<AGN> PrivateBridge<AGN>
where
    AGN: Agent,
    <AGN as Agent>::Input: Serialize + for<'de> Deserialize<'de>,
    <AGN as Agent>::Output: Serialize + for<'de> Deserialize<'de>,
{
    /// Send a message to the worker, queuing it up if necessary
    fn send_message(&self, msg: ToWorker<AGN::Input>) {
        QUEUE.with(|queue| {
            if queue.is_worker_loaded(&TypeId::of::<AGN>()) {
                send_to_remote::<AGN>(&self.worker, msg);
            } else {
                queue.msg_to_queue(msg.pack(), TypeId::of::<AGN>());
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
            queue.remove_agent(&TypeId::of::<AGN>());
        });
    }
}
