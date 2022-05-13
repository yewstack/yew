#![cfg(feature = "hydration")]
#![cfg(target_arch = "wasm32")]

use std::clone::Clone;
use std::cmp::PartialEq;

mod common;

use common::{use_trigger, TriggerBus};
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew::suspense::SuspensionResult;
use yew::tests::{TestCase, TestRunner};
use yew::{use_state, Callback, NodeRef};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

// If any of the assertions fail due to a modification to hydration logic, cargo will suggest the
// expected result and you can copy it into the test to fix it.

#[wasm_bindgen_test]
async fn hydration_works() {
    #[derive(PartialEq, Properties)]
    struct CounterProps {
        button_ref: NodeRef,
    }

    #[function_component]
    fn Counter(CounterProps { button_ref }: &CounterProps) -> Html {
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
                <button {onclick} ref={button_ref}>{"+1"}</button>
            </div>
        }
    }

    let button_ref = NodeRef::default();
    let mut trun = TestRunner::new();
    // no placeholders, hydration is successful.
    trun.prepare_hydrate(html! { <Counter button_ref={&button_ref} /> })
        .await
        .hydrate()
        .await
        .assert_inner_html(r#"<div>Counter: 0<button>+1</button></div>"#);
    button_ref.cast::<HtmlElement>().unwrap().click();
    trun.process_events()
        .await
        .assert_inner_html(r#"<div>Counter: 1<button>+1</button></div>"#);
}

#[wasm_bindgen_test]
async fn hydration_with_suspense() {
    #[derive(PartialEq, Properties, Clone)]
    struct TriggerProps {
        trigger: TriggerBus,
        break_ref: NodeRef,
        inc_ref: NodeRef,
    }

    #[function_component]
    fn Content(props: &TriggerProps) -> HtmlResult {
        let resleep = use_trigger(&props.trigger)?;

        let value = use_state(|| 0);

        let on_increment = {
            let value = value.clone();

            Callback::from(move |_: MouseEvent| {
                value.set(*value + 1);
            })
        };

        let on_take_a_break = Callback::from(move |_: MouseEvent| resleep());

        Ok(html! {
            <div class="content-area">
                <div class="actual-result">{*value}</div>
                <button ref={&props.inc_ref} onclick={on_increment}>{"increase"}</button>
                <div class="action-area">
                    <button ref={&props.break_ref} onclick={on_take_a_break}>{"Take a break!"}</button>
                </div>
            </div>
        })
    }

    #[function_component(App)]
    fn app(props: &TriggerProps) -> Html {
        let fallback = html! {<div>{"wait..."}</div>};

        html! {
            <Suspense {fallback}>
                <Content ..props.clone() />
            </Suspense>
        }
    }
    let mut trun = TestRunner::new();
    let trigger = TriggerBus::new();
    let break_ref = NodeRef::default();
    let inc_ref = NodeRef::default();
    trigger.activate();
    let ssr = trun
        .prepare_hydrate(html! {
            <App trigger={&trigger} break_ref={&break_ref} inc_ref={&inc_ref} />
        })
        .await;
    trigger.deactivate();
    ssr.hydrate()
    .await.assert_inner_html(
        r#"<!--<[hydration::hydration_with_suspense::{{closure}}::Content]>--><div class="content-area"><div class="actual-result">0</div><button>increase</button><div class="action-area"><button>Take a break!</button></div></div><!--</[hydration::hydration_with_suspense::{{closure}}::Content]>-->"#
    );

    trigger.activate();
    trun.process_events().await.assert_inner_html(
        r#"<div class="content-area"><div class="actual-result">0</div><button>increase</button><div class="action-area"><button>Take a break!</button></div></div>"#
    );

    inc_ref.cast::<HtmlElement>().unwrap().click();
    trun.process_events().await.assert_inner_html(
        r#"<div class="content-area"><div class="actual-result">1</div><button>increase</button><div class="action-area"><button>Take a break!</button></div></div>"#
    );

    break_ref.cast::<HtmlElement>().unwrap().click();
    trun.process_events()
        .await
        .assert_inner_html("<div>wait...</div>");

    trigger.activate();
    trun.process_events().await.assert_inner_html(
        r#"<div class="content-area"><div class="actual-result">1</div><button>increase</button><div class="action-area"><button>Take a break!</button></div></div>"#
    );
}

#[wasm_bindgen_test]
async fn hydration_with_suspense_not_suspended_at_start() {
    #[derive(PartialEq, Properties, Clone)]
    struct ContentProps {
        trigger: TriggerBus,
        break_ref: NodeRef,
    }

    #[function_component]
    fn Content(ContentProps { trigger, break_ref }: &ContentProps) -> HtmlResult {
        let resleep = use_trigger(trigger)?;

        let value = use_state(|| "I am writing a long story...".to_string());

        let on_text_input = {
            let value = value.clone();

            Callback::from(move |e: InputEvent| {
                let input: HtmlTextAreaElement = e.target_unchecked_into();

                value.set(input.value());
            })
        };

        let on_take_a_break = Callback::from(move |_| resleep());

        Ok(html! {
            <div class="content-area">
                <textarea value={value.to_string()} oninput={on_text_input}></textarea>
                <div class="action-area">
                    <button ref={break_ref} onclick={on_take_a_break}>{"Take a break!"}</button>
                </div>
            </div>
        })
    }

    #[function_component]
    fn App(props: &ContentProps) -> Html {
        let fallback = html! {<div>{"wait..."}</div>};

        html! {
            <Suspense {fallback}>
                <Content ..props.clone() />
            </Suspense>
        }
    }

    let mut trun = TestRunner::new();
    let trigger = TriggerBus::new();
    let break_ref = NodeRef::default();
    trigger.activate();
    trun.prepare_hydrate(html! {
        <App trigger={&trigger} break_ref={&break_ref} />
    })
    .await
    .hydrate()
    .await.assert_inner_html(
        r#"<div class="content-area"><textarea>I am writing a long story...</textarea><div class="action-area"><button>Take a break!</button></div></div>"#
    );

    break_ref.cast::<HtmlElement>().unwrap().click();
    trun.process_events()
        .await
        .assert_inner_html("<div>wait...</div>");

    trigger.activate();
    trun.process_events().await.assert_inner_html(
        r#"<div class="content-area"><textarea>I am writing a long story...</textarea><div class="action-area"><button>Take a break!</button></div></div>"#
    );
}

#[wasm_bindgen_test]
async fn hydration_nested_suspense_works() {
    #[derive(PartialEq, Properties, Clone)]
    struct NestedProps {
        outer_trigger: TriggerBus,
        inner_trigger: TriggerBus,
        outer_break_ref: NodeRef,
        inner_break_ref: NodeRef,
    }

    #[function_component]
    fn InnerContent(props: &NestedProps) -> HtmlResult {
        let resleep = use_trigger(&props.inner_trigger)?;
        let on_take_a_break = Callback::from(move |_: MouseEvent| resleep());

        Ok(html! {
            <div class="content-area">
                <div class="action-area">
                    <button ref={&props.inner_break_ref} onclick={on_take_a_break}>{"Take a break!"}</button>
                </div>
            </div>
        })
    }

    #[function_component]
    fn Content(props: &NestedProps) -> HtmlResult {
        let resleep = use_trigger(&props.outer_trigger)?;
        let on_take_a_break = Callback::from(move |_: MouseEvent| resleep());
        let fallback = html! {<div>{"wait...(inner)"}</div>};

        Ok(html! {
            <div class="content-area">
                <div class="action-area">
                    <button ref={&props.outer_break_ref} onclick={on_take_a_break}>{"Take a break!"}</button>
                </div>
                <Suspense {fallback}>
                    <InnerContent ..props.clone() />
                </Suspense>
            </div>
        })
    }

    #[function_component]
    fn App(props: &NestedProps) -> Html {
        let fallback = html! {<div>{"wait...(outer)"}</div>};

        html! {
            <Suspense {fallback}>
                <Content ..props.clone() />
            </Suspense>
        }
    }
    let mut trun = TestRunner::new();
    let outer_trigger = TriggerBus::new();
    let inner_trigger = TriggerBus::new();
    let outer_break_ref = NodeRef::default();
    let inner_break_ref = NodeRef::default();

    outer_trigger.activate();
    inner_trigger.activate(); // for ssr
    let ssr = trun.prepare_hydrate(html! {
        <App outer_trigger={&outer_trigger} inner_trigger={&inner_trigger} outer_break_ref={&outer_break_ref} inner_break_ref={&inner_break_ref} />
    }).await;
    outer_trigger.deactivate();
    inner_trigger.deactivate();
    ssr.hydrate()
    .await.assert_inner_html(
        r#"<!--<[hydration::hydration_nested_suspense_works::{{closure}}::Content]>--><div class="content-area"><div class="action-area"><button>Take a break!</button></div><!--<[yew::suspense::component::feat_csr_ssr::Suspense]>--><!--<[yew::suspense::component::feat_csr_ssr::BaseSuspense]>--><!--<?>--><!--<[hydration::hydration_nested_suspense_works::{{closure}}::InnerContent]>--><div class="content-area"><div class="action-area"><button>Take a break!</button></div></div><!--</[hydration::hydration_nested_suspense_works::{{closure}}::InnerContent]>--><!--</?>--><!--</[yew::suspense::component::feat_csr_ssr::BaseSuspense]>--><!--</[yew::suspense::component::feat_csr_ssr::Suspense]>--></div><!--</[hydration::hydration_nested_suspense_works::{{closure}}::Content]>-->"#
    );

    outer_trigger.activate();
    // inner suspense is hydrating...
    trun.process_events().await.assert_inner_html(
        r#"<div class="content-area"><div class="action-area"><button>Take a break!</button></div><!--<[hydration::hydration_nested_suspense_works::{{closure}}::InnerContent]>--><div class="content-area"><div class="action-area"><button>Take a break!</button></div></div><!--</[hydration::hydration_nested_suspense_works::{{closure}}::InnerContent]>--></div>"#
    );

    inner_trigger.activate();
    // hydrated.
    trun.process_events().await.assert_inner_html(
        r#"<div class="content-area"><div class="action-area"><button>Take a break!</button></div><div class="content-area"><div class="action-area"><button>Take a break!</button></div></div></div>"#
    );

    outer_break_ref.cast::<HtmlElement>().unwrap().click();
    trun.process_events()
        .await
        .assert_inner_html("<div>wait...(outer)</div>");

    outer_trigger.activate();
    trun.process_events().await.assert_inner_html(
        r#"<div class="content-area"><div class="action-area"><button>Take a break!</button></div><div class="content-area"><div class="action-area"><button>Take a break!</button></div></div></div>"#
    );

    inner_break_ref.cast::<HtmlElement>().unwrap().click();
    trun.process_events().await.assert_inner_html(
        r#"<div class="content-area"><div class="action-area"><button>Take a break!</button></div><div>wait...(inner)</div></div>"#
    );

    inner_trigger.activate();
    trun.process_events().await.assert_inner_html(
        r#"<div class="content-area"><div class="action-area"><button>Take a break!</button></div><div class="content-area"><div class="action-area"><button>Take a break!</button></div></div></div>"#
    );
}

#[wasm_bindgen_test]
async fn hydration_node_ref_works() {
    #[function_component]
    fn Test1() -> Html {
        html! {
            <span>{"test"}</span>
        }
    }
    #[function_component]
    fn Test2() -> Html {
        html! {
            <Test1/>
        }
    }
    #[derive(Properties, PartialEq)]
    struct ListProps {
        size: u32,
    }
    #[function_component]
    fn List(props: &ListProps) -> Html {
        (0..props.size)
            .map(|_| {
                html! {
                    <Test2/>
                }
            })
            .collect()
    }
    #[derive(Properties, PartialEq)]
    struct AppProps {
        click_ref: NodeRef,
    }
    #[function_component]
    fn App(AppProps { click_ref }: &AppProps) -> Html {
        let size = use_state(|| 4);

        let callback = {
            let size = size.clone();
            Callback::from(move |_| size.set(10))
        };

        html! {
            <div ref={click_ref} onclick={callback}>
                <List size={*size}/>
            </div>
        }
    }

    let mut trun = TestRunner::new();
    let click_ref = NodeRef::default();
    trun.prepare_hydrate(html! {
        <App click_ref={&click_ref} />
    })
    .await
    .hydrate()
    .await
    .assert_inner_html(
        r#"<div><span>test</span><span>test</span><span>test</span><span>test</span></div>"#,
    );

    click_ref.cast::<HtmlElement>().unwrap().click();
    trun.process_events().await.assert_inner_html(
        r#"<div><span>test</span><span>test</span><span>test</span><span>test</span><span>test</span><span>test</span><span>test</span><span>test</span><span>test</span><span>test</span></div>"#
    );
}

#[derive(Properties, PartialEq, Clone)]
struct NumberProps {
    number: u32,
    trigger: TriggerBus,
}

#[function_component]
fn Number(props: &NumberProps) -> Html {
    html! {
        <div>{props.number}</div>
    }
}

#[hook]
pub fn use_suspend(trigger: &TriggerBus) -> SuspensionResult<()> {
    let _ = use_trigger(trigger)?;
    Ok(())
}

#[function_component]
fn SuspendedNumber(props: &NumberProps) -> HtmlResult {
    use_suspend(&props.trigger)?;
    Ok(html! {
        <div>{props.number}</div>
    })
}

#[function_component]
fn ToSuspendOrNot(props: &NumberProps) -> Html {
    let number = props.number;
    html! {
        <Suspense>
            if number % 3 == 0 {
                <SuspendedNumber ..props.clone()/>
            } else {
                <Number ..props.clone()/>
            }
        </Suspense>
    }
}

#[wasm_bindgen_test]
async fn hydration_list_order_works() {
    #[derive(Properties, PartialEq)]
    struct AppProps {
        trigger: TriggerBus,
    }
    #[function_component]
    fn App(AppProps { trigger }: &AppProps) -> Html {
        (0..10)
            .map(|number| {
                html! {
                    <ToSuspendOrNot {number} {trigger} />
                }
            })
            .collect()
    }

    let mut trun = TestRunner::new();
    let trigger = TriggerBus::new();
    trigger.activate(); // for ssr
    let ssr = trun
        .prepare_hydrate(html! {
            <App trigger={&trigger} />
        })
        .await;
    trigger.deactivate();
    ssr.hydrate().await;
    // Reveal all suspended components.
    trigger.activate();
    // Until all components become revealed, there will be component markers.
    // As long as there's no component markers all components have become unsuspended.
    trun.process_events().await.assert_inner_html(
        r#"<div>0</div><div>1</div><div>2</div><div>3</div><div>4</div><div>5</div><div>6</div><div>7</div><div>8</div><div>9</div>"#
    );
}

#[wasm_bindgen_test]
async fn hydration_suspense_no_flickering() {
    #[derive(PartialEq, Properties, Clone)]
    struct SuspendProps {
        inner_bus: TriggerBus,
        outer_bus: TriggerBus,
    }
    #[derive(Properties, PartialEq, Clone)]
    struct NumberProps {
        number: u32,
        busses: SuspendProps,
    }
    #[function_component]
    fn Number(props: &NumberProps) -> Html {
        html! {
            <div>
                {props.number.to_string()}
            </div>
        }
    }
    #[function_component]
    fn SuspendedNumber(props: &NumberProps) -> HtmlResult {
        use_suspend(&props.busses.inner_bus)?;

        Ok(html! {
            <Number ..{props.clone()}/>
        })
    }
    #[function_component]
    fn Suspended(props: &SuspendProps) -> HtmlResult {
        use_suspend(&props.outer_bus)?;

        Ok(html! {
            { for (0..10).map(|number|
                html! {
                    <SuspendedNumber {number} busses={props.clone()} />
                }
            )}
        })
    }
    #[function_component]
    fn App(props: &SuspendProps) -> Html {
        let fallback = html! { <h1>{"Loading..."}</h1> };
        html! {
            <Suspense {fallback}>
                <Suspended ..props.clone() />
            </Suspense>
        }
    }

    let mut trun = TestRunner::new();
    let inner_bus = TriggerBus::new();
    let outer_bus = TriggerBus::new();

    inner_bus.activate();
    outer_bus.activate(); // for ssr
    let ssr = trun
        .prepare_hydrate(html! {
            <App inner_bus={&inner_bus} outer_bus={&outer_bus} />
        })
        .await;
    inner_bus.deactivate();
    outer_bus.deactivate();

    ssr.hydrate().await.assert_inner_html(
        r#"<!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Suspended]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>0</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>1</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>2</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>3</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>4</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>5</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>6</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>7</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>8</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>9</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Suspended]>-->"#
    );

    outer_bus.activate();
    trun.process_events().await.assert_inner_html(
        r#"<!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Suspended]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>0</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>1</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>2</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>3</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>4</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>5</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>6</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>7</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>8</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--<[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><div>9</div><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Number]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::SuspendedNumber]>--><!--</[hydration::hydration_suspense_no_flickering::{{closure}}::Suspended]>-->"#
    );

    inner_bus.activate();
    // inner revealed.
    trun.process_events().await.assert_inner_html(
        r#"<div>0</div><div>1</div><div>2</div><div>3</div><div>4</div><div>5</div><div>6</div><div>7</div><div>8</div><div>9</div>"#
    );
}

#[wasm_bindgen_test]
async fn hydration_order_issue_nested_suspense() {
    #[derive(Properties, PartialEq)]
    struct AppProps {
        trigger: TriggerBus,
    }
    #[function_component]
    fn App(AppProps { trigger }: &AppProps) -> Html {
        html! {
            <Suspense>
                { for (0..10).map(|number: u32| {
                    html! {
                        <ToSuspendOrNot {number} {trigger} />
                    }
                })}
            </Suspense>
        }
    }

    let mut trun = TestRunner::new();
    let trigger = TriggerBus::new();
    trigger.activate(); // for ssr
    let ssr = trun
        .prepare_hydrate(html! {
            <App trigger={&trigger} />
        })
        .await;
    trigger.deactivate();
    ssr.hydrate().await;
    trigger.activate();
    // Until all components become revealed, there will be component markers.
    // As long as there's no component markers all components have become unsuspended.
    trun.process_events().await.assert_inner_html(
        r#"<div>0</div><div>1</div><div>2</div><div>3</div><div>4</div><div>5</div><div>6</div><div>7</div><div>8</div><div>9</div>"#
    );
}
