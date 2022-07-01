use std::cell::RefCell;
use std::rc::Rc;

use futures::channel::oneshot;
use gloo_worker::WorkerBridge;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};

use super::traits::{Task, TaskWorker};
use crate::worker::WorkerProviderState;

/// Handle for [use_task]
#[derive(Debug)]
pub struct UseTaskHandle<T>
where
    T: Task + 'static,
{
    state: WorkerProviderState<TaskWorker<T>>,
}

impl<T> UseTaskHandle<T>
where
    T: Task + 'static,
{
    /// Runs a task.
    pub async fn run(&self, input: T::Input) -> T::Output {
        let (tx, rx) = oneshot::channel();

        let tx_cell = RefCell::new(Some(tx));

        let hold_bridge: Rc<RefCell<Option<WorkerBridge<TaskWorker<T>>>>> = Rc::default();

        let bridge = {
            let hold_bridge = hold_bridge.clone();
            self.state.create_bridge(move |output| {
                if let Some(tx) = tx_cell.borrow_mut().take() {
                    let _ = tx.send(output);
                }

                hold_bridge.borrow_mut().take();
            })
        };

        bridge.send(input);

        *hold_bridge.borrow_mut() = Some(bridge);

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
            state: self.state.clone(),
        }
    }
}

impl<T> PartialEq for UseTaskHandle<T>
where
    T: Task,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.state == rhs.state
    }
}

/// A hook to connect to a task.
#[hook]
pub fn use_task<T>() -> UseTaskHandle<T>
where
    T: Task + 'static,
{
    let state =
        use_context::<WorkerProviderState<TaskWorker<T>>>().expect("failed to find worker context");

    UseTaskHandle { state }
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
