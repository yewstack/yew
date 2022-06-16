use yew::prelude::*;

use super::traits::{Task, TaskWorker};
use crate::worker::{Bincode, Codec, WorkerProvider, WorkerProviderProps};

/// The task provider.
#[function_component]
pub fn TaskProvider<T, CODEC = Bincode>(props: &WorkerProviderProps) -> Html
where
    T: 'static + Task,
    CODEC: Codec + 'static,
{
    let WorkerProviderProps {
        children,
        lazy,
        reach,
        path,
    } = props.clone();

    html! {
        <WorkerProvider<TaskWorker<T>, CODEC> {lazy} {path} {reach}>
            {children}
        </WorkerProvider<TaskWorker<T>, CODEC>>
    }
}
