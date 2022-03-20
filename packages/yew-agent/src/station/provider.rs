use yew::prelude::*;

use super::traits::{Station, StationWorker};
use crate::worker::{WorkerProvider, WorkerProviderProps};

/// The station provider.
#[function_component]
pub fn StationProvider<T>(props: &WorkerProviderProps) -> Html
where
    T: 'static + Station,
{
    let WorkerProviderProps {
        children,
        lazy,
        reach,
        path,
    } = props.clone();

    html! {
        <WorkerProvider<StationWorker<T>> {lazy} {path} {reach}>
            {children}
        </WorkerProvider<StationWorker<T>>>
    }
}
