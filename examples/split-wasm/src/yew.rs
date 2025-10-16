use std::future::{pending, Future};

use web_sys::HtmlInputElement;
use yew::lazy::declare_lazy_component;
use yew::prelude::*;
use yew::suspense::Suspension;
use yew::Renderer;

use super::{COUNTER, GLOBAL_FOO};

// ---------------------------------------------------------------------------
// A counter component — the one we'll load lazily.
// Uses use_state, which triggers re-renders via the scope it was created with.
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CounterProps {
    pub label: AttrValue,
}

#[component]
fn Counter(props: &CounterProps) -> Html {
    let count = use_state(|| 0_i32);
    let onclick = {
        let count = count.clone();
        Callback::from(move |_| count.set(*count + 1))
    };
    let global_foo = GLOBAL_FOO.with(|f| *f);
    let render_counter = COUNTER.with(|cnt| {
        let c = cnt.get();
        cnt.set(c + 1);
        c
    });

    html! {
        <div class="counter">
            <p>{"This component is loaded from a separate bundle, render count: "}{render_counter}</p>
            <p>{"Here is a number loaded from (shared) memory: "}{global_foo}</p>
            <h3>{ &props.label }</h3>
            <p class="count">{ *count }</p>
            <button {onclick}>{ "+1" }</button>
        </div>
    }
}

declare_lazy_component!(Counter as LazyAddition in lazy_addition);

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
            if show { <LazyAddition label="A lazily loaded counter" /> } else { <Pending /> }
        </Suspense>
        </>
    }
}

pub fn main() {
    let _ = Renderer::<App>::new().render();
}
