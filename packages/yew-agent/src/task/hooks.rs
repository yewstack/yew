use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};

use futures::channel::oneshot;
use wasm_bindgen::prelude::*;
use wasm_bindgen::throw_str;
use wasm_bindgen_futures::spawn_local;

use super::traits::{Task, TaskWorker};
use crate::worker::{use_worker_bridge, UseWorkerBridgeHandle};

/// Handle for [use_task]
#[derive(Debug)]
pub struct UseTaskHandle<T>
where
    T: Task + 'static,
{
    ctr: Rc<AtomicUsize>,
    output_handles: Rc<RefCell<HashMap<usize, oneshot::Sender<T::Output>>>>,
    bridge: UseWorkerBridgeHandle<TaskWorker<T>>,
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

impl<T> PartialEq for UseTaskHandle<T>
where
    T: Task,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.bridge == rhs.bridge
    }
}

/// A hook to connect to a task.
#[hook]
pub fn use_task<T>() -> UseTaskHandle<T>
where
    T: Task + 'static,
{
    let ctr = use_memo(|_| AtomicUsize::new(0), ());
    let output_handles = use_mut_ref(HashMap::<usize, oneshot::Sender<T::Output>>::new);

    let bridge = {
        let output_handles = output_handles.clone();

        use_worker_bridge::<TaskWorker<T>, _>(move |(id, output)| {
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

/// A hook to run a task and suspends the component while the task is running.
///
/// The output is memorised and updated when the input changes.
#[hook]
pub fn use_memorized_task<T>(input: T::Input) -> SuspensionResult<Rc<T::Output>>
where
    T: Task + 'static,
    T::Input: Clone + PartialEq,
{
    let task_runner = use_task::<T>();
    let suspension_state = use_state(|| {
        let (suspension, handle) = Suspension::new();

        (Rc::new(RefCell::new(Some(handle))), Err(suspension))
    });

    let (handle, result) = (*suspension_state).clone();

    use_effect_with_deps(
        move |(task_runner, input)| {
            let task_runner = task_runner.clone();
            let input = input.clone();
            spawn_local(async move {
                let output = task_runner.run(input).await;

                if let Some(m) = handle.borrow_mut().take() {
                    suspension_state.set((Rc::default(), Ok(Rc::new(output))));

                    m.resume();
                }
            });
            || {}
        },
        (task_runner, input),
    );

    result
}
