use std::future::{pending, Future};

use wasm_split_helpers::wasm_split as split;
use web_sys::HtmlInputElement;
use yew::lazy::declare_lazy_component;
use yew::prelude::*;
use yew::suspense::Suspension;
use yew::Renderer;

use super::{COUNTER, GLOBAL_FOO};

#[component]
fn Addition() -> Html {
    let global_foo = GLOBAL_FOO.with(|f| *f);
    let counter = COUNTER.with(|cnt| {
        let c = cnt.get();
        cnt.set(c + 1);
        c
    });
    html! {
        <p>{"This component is loaded from a separate bundle and displays: "}{global_foo}{", render count: "}{counter}</p>
    }
}

declare_lazy_component!(Addition as LazyAddition in lazy_addition);

#[component]
fn Pending() -> HtmlResult {
    Err(Suspension::from_future(pending()).into())
}

#[component]
fn App() -> Html {
    let toggle = use_state(|| false);
    let show = *toggle;
    html! {
        <>
        <input id="additional" type="checkbox" checked={*toggle} oninput={move |ev: InputEvent| toggle.set(ev.target_unchecked_into::<HtmlInputElement>().checked())} />
        <label for="additional">{"Display additional content"}</label>
        <Suspense fallback={html! { <p>{"not yet loaded"}</p> }}>
            if show { <LazyAddition /> } else { <Pending /> }
        </Suspense>
        </>
    }
}

pub fn main() {
    let _ = Renderer::<App>::new().render();
}
