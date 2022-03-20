use yew::prelude::*;

use super::traits::{Reactor, ReactorStation};
use crate::station::StationProvider;
use crate::worker::WorkerProviderProps;

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
        <StationProvider<ReactorStation<R>> {lazy} {path} {reach}>
            {children}
        </StationProvider<ReactorStation<R>>>
    }
}
