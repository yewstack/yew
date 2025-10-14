use std::future::{pending, Future};
use std::pin::Pin;

use wasm_split_helpers::wasm_split as split;
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

struct LazyAdditionProxy;

impl LazyComponent for LazyAdditionProxy {
    type Underlying = Addition;

    async fn fetch() -> LazyVTable<Self::Underlying> {
        #[split(lazy_addition)]
        fn split_fetch() -> LazyVTable<Addition> {
            LazyVTable::<Addition>::vtable()
        }
        struct F(Option<Pin<Box<dyn Future<Output = LazyVTable<Addition>> + Send>>>);
        impl Future for F {
            type Output = LazyVTable<Addition>;

            fn poll(
                mut self: Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
            ) -> std::task::Poll<Self::Output> {
                self.0
                    .get_or_insert_with(|| Box::pin(split_fetch()))
                    .as_mut()
                    .poll(cx)
            }
        }
        static CACHE: async_once_cell::Lazy<LazyVTable<Addition>, F> =
            async_once_cell::Lazy::new(F(None));
        *Pin::static_ref(&CACHE).await.get_ref()
    }
}

type LazyAddition = Lazy<LazyAdditionProxy>;

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
