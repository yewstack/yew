use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use yew::prelude::*;

use futures::channel::oneshot;
use wasm_bindgen::prelude::*;
use wasm_bindgen::throw_str;

use super::traits::{Task, TaskWorker};
use crate::worker::{use_bridge, UseBridgeHandle};

/// Handle for [use_task]
#[derive(Debug)]
pub struct UseTaskHandle<T>
where
    T: Task + 'static,
{
    ctr: Rc<AtomicUsize>,
    output_handles: Rc<RefCell<HashMap<usize, oneshot::Sender<T::Output>>>>,
    bridge: UseBridgeHandle<TaskWorker<T>>,
}

impl<T> UseTaskHandle<T>
where
    T: Task + 'static,
{
    /// Runs a task.
    pub async fn run(&self, input: T::Input) -> T::Output {
        let (tx, rx) = oneshot::channel();

        let input_id = self.ctr.fetch_add(1, Ordering::Relaxed);

        {
            let mut output_handles = self.output_handles.borrow_mut();
            output_handles.insert(input_id, tx);
        }

        self.bridge.send((input_id, input));

        rx.await
            .expect_throw("failed to retrieve output of the task")
    }
}

impl<T> Clone for UseTaskHandle<T>
where
    T: Task + 'static,
{
    fn clone(&self) -> Self {
        Self {
            bridge: self.bridge.clone(),
            ctr: self.ctr.clone(),
            output_handles: self.output_handles.clone(),
        }
    }
}

#[hook]
pub fn use_task<T>() -> UseTaskHandle<T>
where
    T: Task + 'static,
{
    let ctr = use_memo(|_| AtomicUsize::new(0), ());
    let output_handles = use_mut_ref(HashMap::<usize, oneshot::Sender<T::Output>>::new);

    let bridge = {
        let output_handles = output_handles.clone();

        use_bridge::<TaskWorker<T>, _>(move |(id, output)| {
            if let Some(m) = {
                let mut output_handles = output_handles.borrow_mut();
                output_handles.remove(&id)
            } {
                m.send(output)
                    .unwrap_or_else(|_| throw_str("failed to send output of the task"));
            }
        })
    };

    UseTaskHandle {
        bridge,
        ctr,
        output_handles,
    }
}
