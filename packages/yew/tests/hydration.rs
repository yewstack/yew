#![cfg(feature = "hydration")]

use std::rc::Rc;
use std::time::Duration;

mod common;

use common::{obtain_result, obtain_result_by_id};

use gloo::timers::future::sleep;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};
use yew::{Renderer, ServerRenderer};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn hydration_works() {
    #[function_component]
    fn Comp() -> Html {
        let ctr = use_state_eq(|| 0);

        let onclick = {
            let ctr = ctr.clone();

            Callback::from(move |_| {
                ctr.set(*ctr + 1);
            })
        };

        html! {
            <div>
                {"Counter: "}{*ctr}
                <button {onclick} class="increase">{"+1"}</button>
            </div>
        }
    }

    #[function_component]
    fn App() -> Html {
        html! {
            <div>
                <Comp />
            </div>
        }
    }

    let s = ServerRenderer::<App>::new().render().await;

    gloo::utils::document()
        .query_selector("#output")
        .unwrap()
        .unwrap()
        .set_inner_html(&s);

    sleep(Duration::ZERO).await;

    Renderer::<App>::with_root(gloo_utils::document().get_element_by_id("output").unwrap())
        .hydrate();

    sleep(Duration::ZERO).await;

    let result = obtain_result_by_id("output");

    // no placeholders, hydration is successful.
    assert_eq!(
        result,
        r#"<div><div>Counter: 0<button class="increase">+1</button></div></div>"#
    );

    gloo_utils::document()
        .query_selector(".increase")
        .unwrap()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .click();

    sleep(Duration::ZERO).await;

    let result = obtain_result_by_id("output");

    assert_eq!(
        result,
        r#"<div><div>Counter: 1<button class="increase">+1</button></div></div>"#
    );
}

#[wasm_bindgen_test]
async fn hydration_with_suspense() {
    #[derive(PartialEq)]
    pub struct SleepState {
        s: Suspension,
    }

    impl SleepState {
        fn new() -> Self {
            let (s, handle) = Suspension::new();

            spawn_local(async move {
                sleep(Duration::from_millis(50)).await;

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

    #[hook]
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

    let s = ServerRenderer::<App>::new().render().await;

    gloo::utils::document()
        .query_selector("#output")
        .unwrap()
        .unwrap()
        .set_inner_html(&s);

    sleep(Duration::ZERO).await;

    Renderer::<App>::with_root(gloo_utils::document().get_element_by_id("output").unwrap())
        .hydrate();

    sleep(Duration::from_millis(10)).await;

    let result = obtain_result();

    // still hydrating, during hydration, the server rendered result is shown.
    assert_eq!(
        result.as_str(),
        r#"<!--<[hydration::hydration_with_suspense::{{closure}}::Content]>--><div class="content-area"><div class="actual-result">0</div><button class="increase">increase</button><div class="action-area"><button class="take-a-break">Take a break!</button></div></div><!--</[hydration::hydration_with_suspense::{{closure}}::Content]>-->"#
    );

    sleep(Duration::from_millis(50)).await;

    let result = obtain_result();

    // hydrated.
    assert_eq!(
        result.as_str(),
        r#"<div class="content-area"><div class="actual-result">0</div><button class="increase">increase</button><div class="action-area"><button class="take-a-break">Take a break!</button></div></div>"#
    );

    gloo_utils::document()
        .query_selector(".increase")
        .unwrap()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .click();

    sleep(Duration::from_millis(50)).await;

    let result = obtain_result();
    assert_eq!(
        result.as_str(),
        r#"<div class="content-area"><div class="actual-result">1</div><button class="increase">increase</button><div class="action-area"><button class="take-a-break">Take a break!</button></div></div>"#
    );

    gloo_utils::document()
        .query_selector(".take-a-break")
        .unwrap()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .click();

    sleep(Duration::from_millis(10)).await;
    let result = obtain_result();
    assert_eq!(result.as_str(), "<div>wait...</div>");

    sleep(Duration::from_millis(50)).await;

    let result = obtain_result();
    assert_eq!(
        result.as_str(),
        r#"<div class="content-area"><div class="actual-result">1</div><button class="increase">increase</button><div class="action-area"><button class="take-a-break">Take a break!</button></div></div>"#
    );
}

#[wasm_bindgen_test]
async fn hydration_with_suspense_not_suspended_at_start() {
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
                sleep(Duration::from_millis(50)).await;

                handle.resume();
            });

            Self { s: Some(s) }.into()
        }
    }

    #[hook]
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

    let s = ServerRenderer::<App>::new().render().await;

    gloo::utils::document()
        .query_selector("#output")
        .unwrap()
        .unwrap()
        .set_inner_html(&s);

    sleep(Duration::ZERO).await;

    Renderer::<App>::with_root(gloo_utils::document().get_element_by_id("output").unwrap())
        .hydrate();

    sleep(Duration::from_millis(10)).await;

    let result = obtain_result();

    assert_eq!(
        result.as_str(),
        r#"<div class="content-area"><textarea>I am writing a long story...</textarea><div class="action-area"><button class="take-a-break">Take a break!</button></div></div>"#
    );
    gloo_utils::document()
        .query_selector(".take-a-break")
        .unwrap()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .click();

    sleep(Duration::from_millis(10)).await;

    let result = obtain_result();
    assert_eq!(result.as_str(), "<div>wait...</div>");

    sleep(Duration::from_millis(50)).await;

    let result = obtain_result();
    assert_eq!(
        result.as_str(),
        r#"<div class="content-area"><textarea>I am writing a long story...</textarea><div class="action-area"><button class="take-a-break">Take a break!</button></div></div>"#
    );
}

#[wasm_bindgen_test]
async fn hydration_nested_suspense_works() {
    #[derive(PartialEq)]
    pub struct SleepState {
        s: Suspension,
    }

    impl SleepState {
        fn new() -> Self {
            let (s, handle) = Suspension::new();

            spawn_local(async move {
                sleep(Duration::from_millis(50)).await;

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

    #[hook]
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

    let s = ServerRenderer::<App>::new().render().await;

    gloo::utils::document()
        .query_selector("#output")
        .unwrap()
        .unwrap()
        .set_inner_html(&s);

    sleep(Duration::ZERO).await;

    Renderer::<App>::with_root(gloo_utils::document().get_element_by_id("output").unwrap())
        .hydrate();

    // outer suspense is hydrating...
    sleep(Duration::from_millis(10)).await;
    let result = obtain_result();
    assert_eq!(
        result.as_str(),
        r#"<!--<[hydration::hydration_nested_suspense_works::{{closure}}::Content]>--><div class="content-area"><div class="action-area"><button class="take-a-break">Take a break!</button></div><!--<[yew::suspense::component::feat_csr_ssr::Suspense]>--><!--<[yew::suspense::component::feat_csr_ssr::BaseSuspense]>--><!--<?>--><!--<[hydration::hydration_nested_suspense_works::{{closure}}::InnerContent]>--><div class="content-area"><div class="action-area"><button class="take-a-break2">Take a break!</button></div></div><!--</[hydration::hydration_nested_suspense_works::{{closure}}::InnerContent]>--><!--</?>--><!--</[yew::suspense::component::feat_csr_ssr::BaseSuspense]>--><!--</[yew::suspense::component::feat_csr_ssr::Suspense]>--></div><!--</[hydration::hydration_nested_suspense_works::{{closure}}::Content]>-->"#
    );

    sleep(Duration::from_millis(50)).await;

    // inner suspense is hydrating...
    let result = obtain_result();
    assert_eq!(
        result.as_str(),
        r#"<div class="content-area"><div class="action-area"><button class="take-a-break">Take a break!</button></div><!--<[hydration::hydration_nested_suspense_works::{{closure}}::InnerContent]>--><div class="content-area"><div class="action-area"><button class="take-a-break2">Take a break!</button></div></div><!--</[hydration::hydration_nested_suspense_works::{{closure}}::InnerContent]>--></div>"#
    );

    sleep(Duration::from_millis(50)).await;

    // hydrated.
    let result = obtain_result();
    assert_eq!(
        result.as_str(),
        r#"<div class="content-area"><div class="action-area"><button class="take-a-break">Take a break!</button></div><div class="content-area"><div class="action-area"><button class="take-a-break2">Take a break!</button></div></div></div>"#
    );

    gloo_utils::document()
        .query_selector(".take-a-break")
        .unwrap()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .click();

    sleep(Duration::from_millis(10)).await;

    let result = obtain_result();
    assert_eq!(result.as_str(), "<div>wait...(outer)</div>");

    sleep(Duration::from_millis(50)).await;

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

    sleep(Duration::from_millis(10)).await;
    let result = obtain_result();
    assert_eq!(
        result.as_str(),
        r#"<div class="content-area"><div class="action-area"><button class="take-a-break">Take a break!</button></div><div>wait...(inner)</div></div>"#
    );

    sleep(Duration::from_millis(50)).await;

    let result = obtain_result();
    assert_eq!(
        result.as_str(),
        r#"<div class="content-area"><div class="action-area"><button class="take-a-break">Take a break!</button></div><div class="content-area"><div class="action-area"><button class="take-a-break2">Take a break!</button></div></div></div>"#
    );
}
