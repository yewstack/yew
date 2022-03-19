use yew::prelude::*;

use super::traits::{Task, TaskWorker};
use crate::worker::{WorkerProvider, WorkerProviderProps};

/// The task provider.
#[function_component]
pub fn TaskProvider<T>(props: &WorkerProviderProps) -> Html
where
    T: 'static + Task,
{
    let WorkerProviderProps {
        children,
        lazy,
        reach,
        path,
    } = props.clone();

    html! {
        <WorkerProvider<TaskWorker<T>> {lazy} {path} {reach}>
            {children}
        </WorkerProvider<TaskWorker<T>>>
    }
}
