mod common;

use common::obtain_result;
use wasm_bindgen_test::*;
use yew::prelude::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

use std::rc::Rc;

use gloo::timers::future::TimeoutFuture;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlElement, HtmlTextAreaElement};
use yew::suspense::{Suspension, SuspensionResult};

#[wasm_bindgen_test]
async fn suspense_works() {
    #[derive(PartialEq)]
    pub struct SleepState {
        s: Suspension,
    }

    impl SleepState {
        fn new() -> Self {
            let (s, handle) = Suspension::new();

            spawn_local(async move {
                TimeoutFuture::new(50).await;

                handle.resume();
            });

            Self { s }
        }
    }

    impl Reducible for SleepState {
        type Action = ();

        fn reduce(self: Rc<Self>, _action: Self::Action) -> Rc<Self> {
            Self::new().into()
        }
    }

    pub fn use_sleep() -> SuspensionResult<Rc<dyn Fn()>> {
        let sleep_state = use_reducer(SleepState::new);

        if sleep_state.s.resumed() {
            Ok(Rc::new(move || sleep_state.dispatch(())))
        } else {
            Err(sleep_state.s.clone())
        }
    }

    #[function_component(Content)]
    fn content() -> HtmlResult {
        let resleep = use_sleep()?;

        let value = use_state(|| 0);

        let on_increment = {
            let value = value.clone();

            Callback::from(move |_: MouseEvent| {
                value.set(*value + 1);
            })
        };

        let on_take_a_break = Callback::from(move |_: MouseEvent| (resleep.clone())());

        Ok(html! {
            <div class="content-area">
                <div class="actual-result">{*value}</div>
                <button class="increase" onclick={on_increment}>{"increase"}</button>
                <div class="action-area">
                    <button class="take-a-break" onclick={on_take_a_break}>{"Take a break!"}</button>
                </div>
            </div>
        })
    }

    #[function_component(App)]
    fn app() -> Html {
        let fallback = html! {<div>{"wait..."}</div>};

        html! {
            <div id="result">
                <Suspense {fallback}>
                    <Content />
                </Suspense>
            </div>
        }
    }

    yew::start_app_in_element::<App>(gloo_utils::document().get_element_by_id("output").unwrap());

    TimeoutFuture::new(10).await;
    let result = obtain_result();
    assert_eq!(result.as_str(), "<div>wait...</div>");

    TimeoutFuture::new(50).await;

    let result = obtain_result();
    assert_eq!(
        result.as_str(),
        r#"<div class="content-area"><div class="actual-result">0</div><button class="increase">increase</button><div class="action-area"><button class="take-a-break">Take a break!</button></div></div>"#
    );

    TimeoutFuture::new(10).await;

    gloo_utils::document()
        .query_selector(".increase")
        .unwrap()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .click();

    gloo_utils::document()
        .query_selector(".increase")
        .unwrap()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .click();

    let result = obtain_result();
    assert_eq!(
        result.as_str(),
        r#"<div class="content-area"><div class="actual-result">2</div><button class="increase">increase</button><div class="action-area"><button class="take-a-break">Take a break!</button></div></div>"#
    );

    gloo_utils::document()
        .query_selector(".take-a-break")
        .unwrap()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .click();

    TimeoutFuture::new(10).await;
    let result = obtain_result();
    assert_eq!(result.as_str(), "<div>wait...</div>");

    TimeoutFuture::new(50).await;

    let result = obtain_result();
    assert_eq!(
        result.as_str(),
        r#"<div class="content-area"><div class="actual-result">2</div><button class="increase">increase</button><div class="action-area"><button class="take-a-break">Take a break!</button></div></div>"#
    );
}

#[wasm_bindgen_test]
async fn suspense_not_suspended_at_start() {
    #[derive(PartialEq)]
    pub struct SleepState {
        s: Option<Suspension>,
    }

    impl SleepState {
        fn new() -> Self {
            Self { s: None }
        }
    }

    impl Reducible for SleepState {
        type Action = ();

        fn reduce(self: Rc<Self>, _action: Self::Action) -> Rc<Self> {
            let (s, handle) = Suspension::new();

            spawn_local(async move {
                TimeoutFuture::new(50).await;

                handle.resume();
            });

            Self { s: Some(s) }.into()
        }
    }

    pub fn use_sleep() -> SuspensionResult<Rc<dyn Fn()>> {
        let sleep_state = use_reducer(SleepState::new);

        let s = match sleep_state.s.clone() {
            Some(m) => m,
            None => return Ok(Rc::new(move || sleep_state.dispatch(()))),
        };

        if s.resumed() {
            Ok(Rc::new(move || sleep_state.dispatch(())))
        } else {
            Err(s)
        }
    }

    #[function_component(Content)]
    fn content() -> HtmlResult {
        let resleep = use_sleep()?;

        let value = use_state(|| "I am writing a long story...".to_string());

        let on_text_input = {
            let value = value.clone();

            Callback::from(move |e: InputEvent| {
                let input: HtmlTextAreaElement = e.target_unchecked_into();

                value.set(input.value());
            })
        };

        let on_take_a_break = Callback::from(move |_| (resleep.clone())());

        Ok(html! {
            <div class="content-area">
                <textarea value={value.to_string()} oninput={on_text_input}></textarea>
                <div class="action-area">
                    <button class="take-a-break" onclick={on_take_a_break}>{"Take a break!"}</button>
                </div>
            </div>
        })
    }

    #[function_component(App)]
    fn app() -> Html {
        let fallback = html! {<div>{"wait..."}</div>};

        html! {
            <div id="result">
                <Suspense {fallback}>
                    <Content />
                </Suspense>
            </div>
        }
    }

    yew::start_app_in_element::<App>(gloo_utils::document().get_element_by_id("output").unwrap());

    TimeoutFuture::new(10).await;

    let result = obtain_result();
    assert_eq!(
        result.as_str(),
        r#"<div class="content-area"><textarea></textarea><div class="action-area"><button class="take-a-break">Take a break!</button></div></div>"#
    );
    gloo_utils::document()
        .query_selector(".take-a-break")
        .unwrap()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .click();

    TimeoutFuture::new(10).await;
    let result = obtain_result();
    assert_eq!(result.as_str(), "<div>wait...</div>");

    TimeoutFuture::new(50).await;

    let result = obtain_result();
    assert_eq!(
        result.as_str(),
        r#"<div class="content-area"><textarea></textarea><div class="action-area"><button class="take-a-break">Take a break!</button></div></div>"#
    );
}

#[wasm_bindgen_test]
async fn suspense_nested_suspense_works() {
    #[derive(PartialEq)]
    pub struct SleepState {
        s: Suspension,
    }

    impl SleepState {
        fn new() -> Self {
            let (s, handle) = Suspension::new();

            spawn_local(async move {
                TimeoutFuture::new(50).await;

                handle.resume();
            });

            Self { s }
        }
    }

    impl Reducible for SleepState {
        type Action = ();

        fn reduce(self: Rc<Self>, _action: Self::Action) -> Rc<Self> {
            Self::new().into()
        }
    }

    pub fn use_sleep() -> SuspensionResult<Rc<dyn Fn()>> {
        let sleep_state = use_reducer(SleepState::new);

        if sleep_state.s.resumed() {
            Ok(Rc::new(move || sleep_state.dispatch(())))
        } else {
            Err(sleep_state.s.clone())
        }
    }

    #[function_component(InnerContent)]
    fn inner_content() -> HtmlResult {
        let resleep = use_sleep()?;

        let on_take_a_break = Callback::from(move |_: MouseEvent| (resleep.clone())());

        Ok(html! {
            <div class="content-area">
                <div class="action-area">
                    <button class="take-a-break2" onclick={on_take_a_break}>{"Take a break!"}</button>
                </div>
            </div>
        })
    }

    #[function_component(Content)]
    fn content() -> HtmlResult {
        let resleep = use_sleep()?;

        let fallback = html! {<div>{"wait...(inner)"}</div>};

        let on_take_a_break = Callback::from(move |_: MouseEvent| (resleep.clone())());

        Ok(html! {
            <div class="content-area">
                <div class="action-area">
                    <button class="take-a-break" onclick={on_take_a_break}>{"Take a break!"}</button>
                </div>
                <Suspense {fallback}>
                    <InnerContent />
                </Suspense>
            </div>
        })
    }

    #[function_component(App)]
    fn app() -> Html {
        let fallback = html! {<div>{"wait...(outer)"}</div>};

        html! {
            <div id="result">
                <Suspense {fallback}>
                    <Content />
                </Suspense>
            </div>
        }
    }

    yew::start_app_in_element::<App>(gloo_utils::document().get_element_by_id("output").unwrap());

    TimeoutFuture::new(10).await;
    let result = obtain_result();
    assert_eq!(result.as_str(), "<div>wait...(outer)</div>");

    TimeoutFuture::new(50).await;

    let result = obtain_result();
    assert_eq!(
        result.as_str(),
        r#"<div class="content-area"><div class="action-area"><button class="take-a-break">Take a break!</button></div><div>wait...(inner)</div></div>"#
    );

    TimeoutFuture::new(50).await;

    let result = obtain_result();
    assert_eq!(
        result.as_str(),
        r#"<div class="content-area"><div class="action-area"><button class="take-a-break">Take a break!</button></div><div class="content-area"><div class="action-area"><button class="take-a-break2">Take a break!</button></div></div></div>"#
    );

    gloo_utils::document()
        .query_selector(".take-a-break2")
        .unwrap()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .click();

    TimeoutFuture::new(10).await;
    let result = obtain_result();
    assert_eq!(
        result.as_str(),
        r#"<div class="content-area"><div class="action-area"><button class="take-a-break">Take a break!</button></div><div>wait...(inner)</div></div>"#
    );

    TimeoutFuture::new(50).await;

    let result = obtain_result();
    assert_eq!(
        result.as_str(),
        r#"<div class="content-area"><div class="action-area"><button class="take-a-break">Take a break!</button></div><div class="content-area"><div class="action-area"><button class="take-a-break2">Take a break!</button></div></div></div>"#
    );
}
