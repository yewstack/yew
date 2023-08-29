#![cfg(target_arch = "wasm32")]

mod common;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use common::obtain_result;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, HtmlTextAreaElement};
use yew::platform::spawn_local;
use yew::platform::time::sleep;
use yew::prelude::*;
use yew::suspense::{use_future, use_future_with, Suspension, SuspensionResult};
use yew::UseStateHandle;

wasm_bindgen_test_configure!(run_in_browser);

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

    yew::Renderer::<App>::with_root(gloo::utils::document().get_element_by_id("output").unwrap())
        .render();

    sleep(Duration::from_millis(10)).await;
    let result = obtain_result();
    assert_eq!(result.as_str(), "<div>wait...</div>");

    sleep(Duration::from_millis(50)).await;

    let result = obtain_result();
    assert_eq!(
        result.as_str(),
        r#"<div class="content-area"><div class="actual-result">0</div><button class="increase">increase</button><div class="action-area"><button class="take-a-break">Take a break!</button></div></div>"#
    );

    sleep(Duration::from_millis(10)).await;

    gloo::utils::document()
        .query_selector(".increase")
        .unwrap()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .click();

    sleep(Duration::ZERO).await;

    gloo::utils::document()
        .query_selector(".increase")
        .unwrap()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .click();

    sleep(Duration::from_millis(1)).await;

    let result = obtain_result();
    assert_eq!(
        result.as_str(),
        r#"<div class="content-area"><div class="actual-result">2</div><button class="increase">increase</button><div class="action-area"><button class="take-a-break">Take a break!</button></div></div>"#
    );

    gloo::utils::document()
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

    yew::Renderer::<App>::with_root(gloo::utils::document().get_element_by_id("output").unwrap())
        .render();

    sleep(Duration::from_millis(10)).await;

    let result = obtain_result();
    assert_eq!(
        result.as_str(),
        r#"<div class="content-area"><textarea></textarea><div class="action-area"><button class="take-a-break">Take a break!</button></div></div>"#
    );
    gloo::utils::document()
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

    yew::Renderer::<App>::with_root(gloo::utils::document().get_element_by_id("output").unwrap())
        .render();

    sleep(Duration::from_millis(10)).await;
    let result = obtain_result();
    assert_eq!(result.as_str(), "<div>wait...(outer)</div>");

    sleep(Duration::from_millis(50)).await;

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

    gloo::utils::document()
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

#[wasm_bindgen_test]
async fn effects_not_run_when_suspended() {
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

    #[derive(Properties, Clone)]
    struct Props {
        counter: Rc<RefCell<u64>>,
    }

    impl PartialEq for Props {
        fn eq(&self, _rhs: &Self) -> bool {
            true
        }
    }

    #[function_component(Content)]
    fn content(props: &Props) -> HtmlResult {
        {
            let counter = props.counter.clone();

            use_effect(move || {
                let mut counter = counter.borrow_mut();

                *counter += 1;

                || {}
            });
        }

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
    fn app(props: &Props) -> Html {
        let fallback = html! {<div>{"wait..."}</div>};

        html! {
            <div id="result">
                <Suspense {fallback}>
                    <Content counter={props.counter.clone()} />
                </Suspense>
            </div>
        }
    }

    let counter = Rc::new(RefCell::new(0_u64));

    let props = Props {
        counter: counter.clone(),
    };

    yew::Renderer::<App>::with_root_and_props(
        gloo::utils::document().get_element_by_id("output").unwrap(),
        props,
    )
    .render();

    sleep(Duration::from_millis(10)).await;
    let result = obtain_result();
    assert_eq!(result.as_str(), "<div>wait...</div>");
    assert_eq!(*counter.borrow(), 0); // effects not called.

    sleep(Duration::from_millis(50)).await;

    let result = obtain_result();
    assert_eq!(
        result.as_str(),
        r#"<div class="content-area"><div class="actual-result">0</div><button class="increase">increase</button><div class="action-area"><button class="take-a-break">Take a break!</button></div></div>"#
    );
    assert_eq!(*counter.borrow(), 1); // effects ran 1 time.

    sleep(Duration::from_millis(10)).await;

    gloo::utils::document()
        .query_selector(".increase")
        .unwrap()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .click();

    sleep(Duration::ZERO).await;

    gloo::utils::document()
        .query_selector(".increase")
        .unwrap()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .click();

    sleep(Duration::from_millis(0)).await;

    let result = obtain_result();
    assert_eq!(
        result.as_str(),
        r#"<div class="content-area"><div class="actual-result">2</div><button class="increase">increase</button><div class="action-area"><button class="take-a-break">Take a break!</button></div></div>"#
    );
    assert_eq!(*counter.borrow(), 3); // effects ran 3 times.

    gloo::utils::document()
        .query_selector(".take-a-break")
        .unwrap()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .click();

    sleep(Duration::from_millis(10)).await;
    let result = obtain_result();
    assert_eq!(result.as_str(), "<div>wait...</div>");
    assert_eq!(*counter.borrow(), 3); // effects ran 3 times.

    sleep(Duration::from_millis(50)).await;

    let result = obtain_result();
    assert_eq!(
        result.as_str(),
        r#"<div class="content-area"><div class="actual-result">2</div><button class="increase">increase</button><div class="action-area"><button class="take-a-break">Take a break!</button></div></div>"#
    );
    assert_eq!(*counter.borrow(), 4); // effects ran 4 times.
}

#[wasm_bindgen_test]
async fn use_suspending_future_works() {
    #[function_component(Content)]
    fn content() -> HtmlResult {
        let _sleep_handle = use_future(|| async move {
            sleep(Duration::from_millis(50)).await;
        })?;

        Ok(html! {
            <div>
                {"Content"}
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

    yew::Renderer::<App>::with_root(gloo::utils::document().get_element_by_id("output").unwrap())
        .render();

    sleep(Duration::from_millis(10)).await;
    let result = obtain_result();
    assert_eq!(result.as_str(), "<div>wait...</div>");

    sleep(Duration::from_millis(50)).await;

    let result = obtain_result();
    assert_eq!(result.as_str(), r#"<div>Content</div>"#);
}

#[wasm_bindgen_test]
async fn use_suspending_future_with_deps_works() {
    #[derive(PartialEq, Properties)]
    struct ContentProps {
        delay_millis: u64,
    }

    #[function_component(Content)]
    fn content(ContentProps { delay_millis }: &ContentProps) -> HtmlResult {
        let delayed_result = use_future_with(*delay_millis, |delay_millis| async move {
            sleep(Duration::from_millis(*delay_millis)).await;
            42
        })?;

        Ok(html! {
            <div>
                {*delayed_result}
            </div>
        })
    }

    #[function_component(App)]
    fn app() -> Html {
        let fallback = html! {<div>{"wait..."}</div>};

        html! {
            <div id="result">
                <Suspense {fallback}>
                    <Content delay_millis={50} />
                </Suspense>
            </div>
        }
    }

    yew::Renderer::<App>::with_root(gloo::utils::document().get_element_by_id("output").unwrap())
        .render();

    sleep(Duration::from_millis(10)).await;
    let result = obtain_result();
    assert_eq!(result.as_str(), "<div>wait...</div>");

    sleep(Duration::from_millis(50)).await;

    let result = obtain_result();
    assert_eq!(result.as_str(), r#"<div>42</div>"#);
}

#[wasm_bindgen_test]
async fn test_suspend_forever() {
    /// A component that its suspension never resumes.
    /// We test that this can be used with to trigger a suspension and unsuspend upon unmount.
    #[function_component]
    fn SuspendForever() -> HtmlResult {
        let (s, handle) = Suspension::new();
        use_state(move || handle);
        Err(s.into())
    }

    #[function_component]
    fn App() -> Html {
        let page = use_state(|| 1);

        {
            let page_setter = page.setter();
            use_effect_with((), move |_| {
                spawn_local(async move {
                    sleep(Duration::from_secs(1)).await;
                    page_setter.set(2);
                });
            });
        }

        let content = if *page == 1 {
            html! { <SuspendForever /> }
        } else {
            html! { <div id="result">{"OK"}</div> }
        };

        html! {
            <Suspense fallback={html! {<div>{"Loading..."}</div>}}>
                {content}
            </Suspense>
        }
    }

    yew::Renderer::<App>::with_root(gloo::utils::document().get_element_by_id("output").unwrap())
        .render();

    sleep(Duration::from_millis(1500)).await;

    let result = obtain_result();
    assert_eq!(result.as_str(), r#"OK"#);
}

#[wasm_bindgen_test]
async fn resume_after_unmount() {
    #[derive(Clone, Properties, PartialEq)]
    struct ContentProps {
        state: UseStateHandle<bool>,
    }

    #[function_component(Content)]
    fn content(ContentProps { state }: &ContentProps) -> HtmlResult {
        let state = state.clone();
        let _sleep_handle = use_future(|| async move {
            sleep(Duration::from_millis(50)).await;
            state.set(false);
            sleep(Duration::from_millis(50)).await;
        })?;

        Ok(html! {
            <div>{"Content"}</div>
        })
    }

    #[function_component(App)]
    fn app() -> Html {
        let fallback = html! {<div>{"wait..."}</div>};
        let state = use_state(|| true);

        html! {
            <div id="result">
            if *state {
                <Suspense {fallback}>
                    <Content {state} />
                </Suspense>
            } else {
                <div>{"Content replacement"}</div>
            }
            </div>
        }
    }

    yew::Renderer::<App>::with_root(gloo::utils::document().get_element_by_id("output").unwrap())
        .render();

    sleep(Duration::from_millis(25)).await;
    let result = obtain_result();
    assert_eq!(result.as_str(), "<div>wait...</div>");

    sleep(Duration::from_millis(50)).await;
    let result = obtain_result();
    assert_eq!(result.as_str(), "<div>Content replacement</div>");
}

#[wasm_bindgen_test]
async fn test_duplicate_suspension() {
    use yew::html::ChildrenProps;

    #[function_component]
    fn FetchingProvider(props: &ChildrenProps) -> HtmlResult {
        use_future(|| async {
            sleep(Duration::ZERO).await;
        })?;
        Ok(html! { <>{props.children.clone()}</> })
    }

    #[function_component]
    fn Child() -> Html {
        html! {<div id="result">{"hello!"}</div>}
    }

    #[function_component]
    fn App() -> Html {
        let fallback = Html::default();
        html! {
           <Suspense {fallback}>
                <FetchingProvider>
                    <Child />
                </FetchingProvider>
           </Suspense>
        }
    }

    yew::Renderer::<App>::with_root(gloo::utils::document().get_element_by_id("output").unwrap())
        .render();

    sleep(Duration::from_millis(50)).await;
    let result = obtain_result();
    assert_eq!(result.as_str(), "hello!");
}
