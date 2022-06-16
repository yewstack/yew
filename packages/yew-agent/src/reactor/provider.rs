use yew::prelude::*;

use super::traits::{Reactor, ReactorWorker};
use crate::worker::{WorkerProvider, WorkerProviderProps};

/// The reactor provider.
#[function_component]
pub fn ReactorProvider<R>(props: &WorkerProviderProps) -> Html
where
    R: 'static + Reactor,
{
    let WorkerProviderProps {
        children,
        lazy,
        reach,
        path,
    } = props.clone();

    html! {
        <WorkerProvider<ReactorWorker<R>> {lazy} {path} {reach}>
            {children}
        </WorkerProvider<ReactorWorker<R>>>
    }
}
