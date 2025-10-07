use std::future::pending;

use wasm_split::wasm_split as split;
use web_sys::HtmlInputElement;
use yew::lazy::{Lazy, LazyComponent, LazyVTable};
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

impl LazyComponent for Addition {
    async fn fetch() -> LazyVTable<Self> {
        #[split(lazy_addition)]
        fn split_fetch() -> LazyVTable<Addition> {
            LazyVTable::<Addition>::vtable()
        }
        split_fetch().await
    }
}

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
            if show { <Lazy<Addition> /> } else { <Pending /> }
        </Suspense>
        </>
    }
}

pub fn main() {
    let _ = Renderer::<App>::new().render();
}
