#![cfg(feature = "hydration")]
#![cfg(target_arch = "wasm32")]

use std::ops::Range;
use std::rc::Rc;
use std::time::Duration;

mod common;

use common::{obtain_result, obtain_result_by_id};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, HtmlTextAreaElement};
use yew::platform::time::sleep;
use yew::prelude::*;
use yew::suspense::{use_future, Suspension, SuspensionResult};
use yew::virtual_dom::VNode;
use yew::{function_component, Renderer, ServerRenderer};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

// If any of the assertions fail due to a modification to hydration logic, cargo will suggest the
// expected result and you can copy it into the test to fix it.

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

    Renderer::<App>::with_root(gloo::utils::document().get_element_by_id("output").unwrap())
        .hydrate();

    sleep(Duration::ZERO).await;

    let result = obtain_result_by_id("output");

    // no placeholders, hydration is successful.
    assert_eq!(
        result,
        r#"<div><div>Counter: 0<button class="increase">+1</button></div></div>"#
    );

    gloo::utils::document()
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
async fn hydration_with_raw() {
    #[function_component(Content)]
    fn content() -> Html {
        let vnode = VNode::from_html_unchecked("<div><p>Hello World</p></div>".into());

        html! {
            <div class="content-area">
                {vnode}
            </div>
        }
    }

    #[function_component(App)]
    fn app() -> Html {
        html! {
            <div id="result">
                <Content />
            </div>
        }
    }

    let s = ServerRenderer::<App>::new().render().await;

    gloo::utils::document()
        .query_selector("#output")
        .unwrap()
        .unwrap()
        .set_inner_html(&s);

    sleep(Duration::from_millis(10)).await;

    Renderer::<App>::with_root(gloo::utils::document().get_element_by_id("output").unwrap())
        .hydrate();

    let result = obtain_result();

    // still hydrating, during hydration, the server rendered result is shown.
    assert_eq!(
        result.as_str(),
        r#"<!--<[hydration::hydration_with_raw::{{closure}}::Content]>--><div class="content-area"><!--<#>--><div><p>Hello World</p></div><!--</#>--></div><!--</[hydration::hydration_with_raw::{{closure}}::Content]>-->"#
    );

    sleep(Duration::from_millis(50)).await;

    let result = obtain_result();

    // hydrated.
    assert_eq!(
        result.as_str(),
        r#"<div class="content-area"><div><p>Hello World</p></div></div>"#
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

    Renderer::<App>::with_root(gloo::utils::document().get_element_by_id("output").unwrap())
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

    gloo::utils::document()
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

    Renderer::<App>::with_root(gloo::utils::document().get_element_by_id("output").unwrap())
        .hydrate();

    sleep(Duration::from_millis(10)).await;

    let result = obtain_result();

    assert_eq!(
        result.as_str(),
        r#"<div class="content-area"><textarea>I am writing a long story...</textarea><div class="action-area"><button class="take-a-break">Take a break!</button></div></div>"#
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

    Renderer::<App>::with_root(gloo::utils::document().get_element_by_id("output").unwrap())
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

    gloo::utils::document()
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
async fn hydration_node_ref_works() {
    #[function_component(App)]
    pub fn app() -> Html {
        let size = use_state(|| 4);

        let callback = {
            let size = size.clone();
            Callback::from(move |_| {
                size.set(10);
            })
        };

        html! {
            <div onclick={callback}>
                <List size={*size}/>
            </div>
        }
    }

    #[derive(Properties, PartialEq)]
    struct ListProps {
        size: u32,
    }

    #[function_component(Test1)]
    fn test1() -> Html {
        html! {
            <span>{"test"}</span>
        }
    }
    #[function_component(Test2)]
    fn test2() -> Html {
        html! {
            <Test1/>
        }
    }

    #[function_component(List)]
    fn list(props: &ListProps) -> Html {
        let elems = 0..props.size;

        html! {
            <>
            { for elems.map(|_|
                html! {
                    <Test2/>
                }
            )}
            </>
        }
    }

    let s = ServerRenderer::<App>::new().render().await;

    gloo::utils::document()
        .query_selector("#output")
        .unwrap()
        .unwrap()
        .set_inner_html(&s);

    sleep(Duration::ZERO).await;

    Renderer::<App>::with_root(gloo::utils::document().get_element_by_id("output").unwrap())
        .hydrate();

    sleep(Duration::ZERO).await;

    let result = obtain_result_by_id("output");
    assert_eq!(
        result.as_str(),
        r#"<div><span>test</span><span>test</span><span>test</span><span>test</span></div>"#
    );

    gloo::utils::document()
        .query_selector("span")
        .unwrap()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .click();

    sleep(Duration::ZERO).await;

    let result = obtain_result_by_id("output");
    assert_eq!(
        result.as_str(),
        r#"<div><span>test</span><span>test</span><span>test</span><span>test</span><span>test</span><span>test</span><span>test</span><span>test</span><span>test</span><span>test</span></div>"#
    );
}

#[wasm_bindgen_test]
async fn hydration_list_order_works() {
    #[function_component(App)]
    pub fn app() -> Html {
        let elems = 0..10;

        html! {
            <>
            { for elems.map(|number|
                html! {
                    <ToSuspendOrNot {number}/>
                }
            )}
            </>
        }
    }

    #[derive(Properties, PartialEq)]
    struct NumberProps {
        number: u32,
    }

    #[function_component(Number)]
    fn number(props: &NumberProps) -> Html {
        html! {
            <div>{props.number.to_string()}</div>
        }
    }
    #[function_component(SuspendedNumber)]
    fn suspended_number(props: &NumberProps) -> HtmlResult {
        use_suspend()?;
        Ok(html! {
            <div>{props.number.to_string()}</div>
        })
    }
    #[function_component(ToSuspendOrNot)]
    fn suspend_or_not(props: &NumberProps) -> Html {
        let number = props.number;
        html! {
            <Suspense>
                if number % 3 == 0 {
                    <SuspendedNumber {number}/>
                } else {
                    <Number {number}/>
                }
            </Suspense>
        }
    }

    #[hook]
    pub fn use_suspend() -> SuspensionResult<()> {
        use_future(|| async {})?;
        Ok(())
    }

    let s = ServerRenderer::<App>::new().render().await;

    gloo::utils::document()
        .query_selector("#output")
        .unwrap()
        .unwrap()
        .set_inner_html(&s);

    sleep(Duration::ZERO).await;

    Renderer::<App>::with_root(gloo::utils::document().get_element_by_id("output").unwrap())
        .hydrate();

    // Wait until all suspended components becomes revealed.
    sleep(Duration::ZERO).await;
    sleep(Duration::ZERO).await;
    sleep(Duration::ZERO).await;
    sleep(Duration::ZERO).await;

    let result = obtain_result_by_id("output");
    assert_eq!(
        result.as_str(),
        // Until all components become revealed, there will be component markers.
        // As long as there's no component markers all components have become unsuspended.
        r#"<div>0</div><div>1</div><div>2</div><div>3</div><div>4</div><div>5</div><div>6</div><div>7</div><div>8</div><div>9</div>"#
    );
}

#[wasm_bindgen_test]
async fn hydration_suspense_no_flickering() {
    #[function_component(App)]
    pub fn app() -> Html {
        let fallback = html! { <h1>{"Loading..."}</h1> };
        html! {
            <Suspense {fallback}>
                <Suspended/>
            </Suspense>
        }
    }

    #[derive(Properties, PartialEq, Clone)]
    struct NumberProps {
        number: u32,
    }

    #[function_component(SuspendedNumber)]
    fn suspended_number(props: &NumberProps) -> HtmlResult {
        use_suspend()?;

        Ok(html! {
            <Number ..{props.clone()}/>
        })
    }
    #[function_component(Number)]
    fn number(props: &NumberProps) -> Html {
        html! {
            <div>
                {props.number.to_string()}
            </div>
        }
    }

    #[function_component(Suspended)]
    fn suspended() -> HtmlResult {
        use_suspend()?;

        Ok(html! {
            { for (0..10).map(|number|
                html! {
                    <SuspendedNumber {number}/>
                }
            )}
        })
    }

    #[hook]
    pub fn use_suspend() -> SuspensionResult<()> {
        use_future(|| async {
            yew::platform::time::sleep(std::time::Duration::from_millis(200)).await;
        })?;
        Ok(())
    }

    let s = ServerRenderer::<App>::new().render().await;

    gloo::utils::document()
        .query_selector("#output")
        .unwrap()
        .unwrap()
        .set_inner_html(&s);

    sleep(Duration::ZERO).await;

    Renderer::<App>::with_root(gloo::utils::document().get_element_by_id("output").unwrap())
        .hydrate();

    // Wait until all suspended components becomes revealed.
    sleep(Duration::ZERO).await;

    let result = obtain_result_by_id("output");
    assert_eq!(
        result.as_str(),
        // outer still suspended.
        r#"<!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Suspended]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>0</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>1</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>2</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>3</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>4</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>5</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>6</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>7</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>8</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>9</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Suspended]>-->"#
    );
    sleep(Duration::from_millis(103)).await;

    let result = obtain_result_by_id("output");
    assert_eq!(
        result.as_str(),
        r#"<!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Suspended]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>0</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>1</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>2</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>3</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>4</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>5</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>6</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>7</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>8</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>9</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Suspended]>-->"#
    );
    sleep(Duration::from_millis(103)).await;

    let result = obtain_result_by_id("output");
    assert_eq!(
        result.as_str(),
        r#"<!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Suspended]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>0</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>1</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>2</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>3</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>4</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>5</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>6</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>7</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>8</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>9</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Suspended]>-->"#
    );
    sleep(Duration::from_millis(103)).await;

    let result = obtain_result_by_id("output");
    assert_eq!(
        result.as_str(),
        // outer revealed, inner still suspended, outer remains.
        r#"<!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Suspended]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>0</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>1</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>2</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>3</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>4</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>5</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>6</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>7</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>8</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>9</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Suspended]>-->"#
    );

    sleep(Duration::from_millis(103)).await;

    let result = obtain_result_by_id("output");
    assert_eq!(
        result.as_str(),
        // inner revealed.
        r#"<div>0</div><div>1</div><div>2</div><div>3</div><div>4</div><div>5</div><div>6</div><div>7</div><div>8</div><div>9</div>"#
    );
}

#[wasm_bindgen_test]
async fn hydration_order_issue_nested_suspense() {
    #[function_component(App)]
    pub fn app() -> Html {
        let elems = (0..10).map(|number: u32| {
            html! {
                <ToSuspendOrNot {number} key={number} />
            }
        });

        html! {
            <Suspense>
                { for elems }
            </Suspense>
        }
    }

    #[derive(Properties, PartialEq)]
    struct NumberProps {
        number: u32,
    }

    #[function_component(Number)]
    fn number(props: &NumberProps) -> Html {
        html! {
            <div>{props.number.to_string()}</div>
        }
    }

    #[function_component(SuspendedNumber)]
    fn suspended_number(props: &NumberProps) -> HtmlResult {
        use_suspend()?;
        Ok(html! {
            <div>{props.number.to_string()}</div>
        })
    }

    #[function_component(ToSuspendOrNot)]
    fn suspend_or_not(props: &NumberProps) -> HtmlResult {
        let number = props.number;
        Ok(html! {
            if number % 3 == 0 {
                <Suspense>
                    <SuspendedNumber {number} />
                </Suspense>
            } else {
                <Number {number} />
            }
        })
    }

    #[hook]
    pub fn use_suspend() -> SuspensionResult<()> {
        use_future(|| async {})?;

        Ok(())
    }

    let s = ServerRenderer::<App>::new().render().await;

    gloo::utils::document()
        .query_selector("#output")
        .unwrap()
        .unwrap()
        .set_inner_html(&s);

    sleep(Duration::ZERO).await;

    Renderer::<App>::with_root(gloo::utils::document().get_element_by_id("output").unwrap())
        .hydrate();

    // Wait until all suspended components becomes revealed.
    sleep(Duration::ZERO).await;
    sleep(Duration::ZERO).await;
    sleep(Duration::ZERO).await;
    sleep(Duration::ZERO).await;

    let result = obtain_result_by_id("output");
    assert_eq!(
        result.as_str(),
        // Until all components become revealed, there will be component markers.
        // As long as there's no component markers all components have become unsuspended.
        r#"<div>0</div><div>1</div><div>2</div><div>3</div><div>4</div><div>5</div><div>6</div><div>7</div><div>8</div><div>9</div>"#
    );
}

#[wasm_bindgen_test]
async fn hydration_props_blocked_until_hydrated() {
    #[function_component(App)]
    pub fn app() -> Html {
        let range = use_state(|| 0u32..2);
        {
            let range = range.clone();
            use_effect_with((), move |_| {
                range.set(0..3);
                || ()
            });
        }

        html! {
            <Suspense>
                <ToSuspend range={(*range).clone()}/>
            </Suspense>
        }
    }

    #[derive(Properties, PartialEq)]
    struct ToSuspendProps {
        range: Range<u32>,
    }

    #[function_component(ToSuspend)]
    fn to_suspend(ToSuspendProps { range }: &ToSuspendProps) -> HtmlResult {
        use_suspend(Duration::from_millis(100))?;
        Ok(html! {
            { for range.clone().map(|i|
                html!{ <div key={i}>{i}</div> }
            )}
        })
    }

    #[hook]
    pub fn use_suspend(_dur: Duration) -> SuspensionResult<()> {
        yew::suspense::use_future(|| async move {
            sleep(_dur).await;
        })?;

        Ok(())
    }

    let s = ServerRenderer::<App>::new().render().await;

    let output_element = gloo::utils::document()
        .query_selector("#output")
        .unwrap()
        .unwrap();

    output_element.set_inner_html(&s);

    Renderer::<App>::with_root(output_element).hydrate();
    sleep(Duration::from_millis(150)).await;

    let result = obtain_result_by_id("output");
    assert_eq!(result.as_str(), r#"<div>0</div><div>1</div><div>2</div>"#);
}

#[wasm_bindgen_test]
async fn hydrate_empty() {
    #[function_component]
    fn Updating() -> Html {
        let trigger = use_state(|| false);
        {
            let trigger = trigger.clone();
            use_effect_with((), move |_| {
                trigger.set(true);
                || {}
            });
        }
        if *trigger {
            html! { <div>{"after"}</div> }
        } else {
            html! { <div>{"before"}</div> }
        }
    }
    #[function_component]
    fn Empty() -> Html {
        html! { <></> }
    }
    #[function_component]
    fn App() -> Html {
        html! {
            <>
                <Updating />
                <Empty />
                <Updating />
            </>
        }
    }
    let s = ServerRenderer::<App>::new().render().await;

    let output_element = gloo::utils::document()
        .query_selector("#output")
        .unwrap()
        .unwrap();

    output_element.set_inner_html(&s);

    Renderer::<App>::with_root(output_element).hydrate();
    sleep(Duration::from_millis(50)).await;

    let result = obtain_result_by_id("output");
    assert_eq!(result.as_str(), r#"<div>after</div><div>after</div>"#);
}
