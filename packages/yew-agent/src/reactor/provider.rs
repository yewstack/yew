use yew::prelude::*;

use super::traits::{Reactor, ReactorWorker};
use crate::worker::{WorkerProvider, WorkerProviderProps};
use crate::{Bincode, Codec};

/// The reactor provider.
#[function_component]
pub fn ReactorProvider<R, CODEC = Bincode>(props: &WorkerProviderProps) -> Html
where
    R: 'static + Reactor,
    CODEC: Codec + 'static,
{
    let WorkerProviderProps {
        children,
        lazy,
        reach,
        path,
    } = props.clone();

    html! {
        <WorkerProvider<ReactorWorker<R>, CODEC> {lazy} {path} {reach}>
            {children}
        </WorkerProvider<ReactorWorker<R>, CODEC>>
    }
}
