use std::thread;

use futures::channel::oneshot;
use yew::platform::{spawn_local, LocalRuntime};

use super::Task;

/// This is not used during client-side rendering, we can avoid a feature flag using dead code
/// elimination.
/// In the future, it might be better to provide a custom procedural macro like `#[yew::cfg_ssr]` to
/// provide consistent configuration.
#[allow(dead_code)]
pub(crate) async fn execute_task<T>(input: T::Input) -> T::Output
where
    T: Task,
    T::Input: Send + Clone + 'static,
    T::Output: Send + 'static,
{
    let (tx, rx) = oneshot::channel();
    let spawn_result = {
        let input = input.clone();
        thread::Builder::new().spawn(move || {
            let rt = LocalRuntime::new().expect("failed to create local runtime.");

            rt.block_on(async move {
                let output = T::run(input).await;
                let _ = tx.send(output);
            })
        })
    };

    // If a thread cannot be spawned, it means that current platform has no thread support.
    // We spawn it to current thread instead.
    if spawn_result.is_ok() {
        rx.await.expect("task has panicked.")
    } else {
        let (tx, rx) = oneshot::channel();
        spawn_local(async move {
            let output = T::run(input).await;
            let _ = tx.send(output);
        });

        rx.await.expect("task has panicked.")
    }
}
