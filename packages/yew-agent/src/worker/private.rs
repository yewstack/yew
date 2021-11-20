use super::*;
use queue::Queue;
use std::cell::RefCell;
use std::fmt;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use web_sys::Worker;
use yew::callback::Callback;

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

        let name_of_resource = AGN::name_of_resource();
        let is_relative = AGN::resource_path_is_relative();
        let handler_cell = Rc::new(RefCell::new(Some(handler)));

        let worker = {
            let handler_cell = handler_cell.clone();
            let worker = worker_new(name_of_resource, is_relative, AGN::is_module());
            let worker_clone = worker.clone();
            worker.set_onmessage_closure(move |data: Vec<u8>| {
                if let Some(handler) = handler_cell.borrow().as_ref() {
                    handler(data, &worker_clone)
                }
            });
            worker
        };
        let bridge = PrivateBridge {
            handler_cell,
            worker,
            _agent: PhantomData,
            id,
        };
        bridge.send_message(ToWorker::Connected(SINGLETON_ID));
        Box::new(bridge)
    }
}

/// A connection manager for components interaction with workers.
pub struct PrivateBridge<AGN, HNDL>
where
    AGN: Agent,
    <AGN as Agent>::Input: Serialize + for<'de> Deserialize<'de>,
    <AGN as Agent>::Output: Serialize + for<'de> Deserialize<'de>,
    HNDL: Fn(Vec<u8>, &Worker),
{
    handler_cell: Rc<RefCell<Option<HNDL>>>,
    worker: Worker,
    _agent: PhantomData<AGN>,
    id: usize,
}

impl<AGN, HNDL> PrivateBridge<AGN, HNDL>
where
    AGN: Agent,
    <AGN as Agent>::Input: Serialize + for<'de> Deserialize<'de>,
    <AGN as Agent>::Output: Serialize + for<'de> Deserialize<'de>,
    HNDL: Fn(Vec<u8>, &Worker),
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

impl<AGN, HNDL> fmt::Debug for PrivateBridge<AGN, HNDL>
where
    AGN: Agent,
    <AGN as Agent>::Input: Serialize + for<'de> Deserialize<'de>,
    <AGN as Agent>::Output: Serialize + for<'de> Deserialize<'de>,
    HNDL: Fn(Vec<u8>, &Worker),
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PrivateBridge<_>")
    }
}

impl<AGN, HNDL> Bridge<AGN> for PrivateBridge<AGN, HNDL>
where
    AGN: Agent,
    <AGN as Agent>::Input: Serialize + for<'de> Deserialize<'de>,
    <AGN as Agent>::Output: Serialize + for<'de> Deserialize<'de>,
    HNDL: Fn(Vec<u8>, &Worker),
{
    fn send(&mut self, msg: AGN::Input) {
        let msg = ToWorker::ProcessInput(SINGLETON_ID, msg);
        self.send_message(msg);
    }
}

impl<AGN, HNDL> Drop for PrivateBridge<AGN, HNDL>
where
    AGN: Agent,
    <AGN as Agent>::Input: Serialize + for<'de> Deserialize<'de>,
    <AGN as Agent>::Output: Serialize + for<'de> Deserialize<'de>,
    HNDL: Fn(Vec<u8>, &Worker),
{
    fn drop(&mut self) {
        let disconnected = ToWorker::Disconnected(SINGLETON_ID);
        send_to_remote::<AGN>(&self.worker, disconnected);

        let destroy = ToWorker::Destroy;
        send_to_remote::<AGN>(&self.worker, destroy);

        self.handler_cell.borrow_mut().take();

        QUEUE.with(|queue| {
            queue.remove_agent(&self.id);
        });
    }
}
