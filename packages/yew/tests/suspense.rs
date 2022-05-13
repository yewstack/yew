#![cfg(target_arch = "wasm32")]

mod common;

use common::{use_trigger, TriggerBus};
use wasm_bindgen_test::*;
use yew::prelude::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

use std::cell::RefCell;
use std::rc::Rc;

use web_sys::HtmlElement;
use yew::suspense::{use_future, use_future_with_deps};
use yew::tests::{TestCase, TestRunner};

#[wasm_bindgen_test]
async fn suspense_works() {
    #[derive(PartialEq, Properties, Clone)]
    struct AppProps {
        inc_ref: NodeRef,
        break_ref: NodeRef,
        resleep_bus: TriggerBus,
    }
    #[function_component]
    fn Content(props: &AppProps) -> HtmlResult {
        let resleep = use_trigger(&props.resleep_bus)?;
        let value = use_state(|| 0);

        let on_increment = {
            let value = value.clone();

            Callback::from(move |_: MouseEvent| value.set(*value + 1))
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

    #[function_component]
    fn App(props: &AppProps) -> Html {
        let fallback = html! {<div>{"wait..."}</div>};

        html! {
            <Suspense {fallback}>
                <Content ..props.clone() />
            </Suspense>
        }
    }

    let mut trun = TestRunner::new();
    let break_ref = NodeRef::default();
    let inc_ref = NodeRef::default();
    let resleep_bus = TriggerBus::new();
    trun.render(html! {
        <App resleep_bus={&resleep_bus} break_ref={&break_ref} inc_ref={&inc_ref} />
    })
    .await
    .assert_inner_html("<div>wait...</div>");

    resleep_bus.activate();
    trun.process_events().await.assert_inner_html(
        r#"<div class="content-area"><div class="actual-result">0</div><button>increase</button><div class="action-area"><button>Take a break!</button></div></div>"#
    );

    inc_ref.cast::<HtmlElement>().unwrap().click();
    trun.process_events().await;
    inc_ref.cast::<HtmlElement>().unwrap().click();

    trun.process_events().await.assert_inner_html(
        r#"<div class="content-area"><div class="actual-result">2</div><button>increase</button><div class="action-area"><button>Take a break!</button></div></div>"#
    );

    break_ref.cast::<HtmlElement>().unwrap().click();
    trun.process_events()
        .await
        .assert_inner_html("<div>wait...</div>");

    resleep_bus.activate();
    trun.process_events().await.assert_inner_html(
        r#"<div class="content-area"><div class="actual-result">2</div><button>increase</button><div class="action-area"><button>Take a break!</button></div></div>"#
    );
}

#[wasm_bindgen_test]
async fn suspense_not_suspended_at_start() {
    #[derive(PartialEq, Properties, Clone)]
    struct AppProps {
        break_ref: NodeRef,
        resleep_bus: TriggerBus,
    }
    #[function_component]
    fn Content(props: &AppProps) -> HtmlResult {
        let resleep = use_trigger(&props.resleep_bus)?;
        let value = use_state(|| "I am writing a long story...".to_string());

        let on_take_a_break = Callback::from(move |_| resleep());

        Ok(html! {
            <div class="content-area">
                <textarea value={value.to_string()}></textarea>
                <div class="action-area">
                    <button ref={&props.break_ref} onclick={on_take_a_break}>{"Take a break!"}</button>
                </div>
            </div>
        })
    }
    #[function_component]
    fn App(props: &AppProps) -> Html {
        let fallback = html! {<div>{"wait..."}</div>};

        html! {
            <Suspense {fallback}>
                <Content ..props.clone() />
            </Suspense>
        }
    }

    let mut trun = TestRunner::new();
    let break_ref = NodeRef::default();
    let resleep_bus = TriggerBus::new();
    resleep_bus.activate();
    trun.render(html! {
        <App resleep_bus={&resleep_bus} break_ref={&break_ref} />
    })
    .await
    .assert_inner_html(
        r#"<div class="content-area"><textarea></textarea><div class="action-area"><button>Take a break!</button></div></div>"#
    );

    break_ref.cast::<HtmlElement>().unwrap().click();
    trun.process_events()
        .await
        .assert_inner_html("<div>wait...</div>");

    resleep_bus.activate();
    trun.process_events().await.assert_inner_html(
        r#"<div class="content-area"><textarea></textarea><div class="action-area"><button>Take a break!</button></div></div>"#
    );
}

#[wasm_bindgen_test]
async fn suspense_nested_suspense_works() {
    #[derive(PartialEq, Properties, Clone)]
    struct AppProps {
        outer_trigger: TriggerBus,
        inner_trigger: TriggerBus,
        outer_break_ref: NodeRef,
        inner_break_ref: NodeRef,
    }
    #[function_component]
    fn InnerContent(props: &AppProps) -> HtmlResult {
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
    fn Content(props: &AppProps) -> HtmlResult {
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
    fn App(props: &AppProps) -> Html {
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

    trun.render(html! {
        <App outer_trigger={&outer_trigger} inner_trigger={&inner_trigger} outer_break_ref={&outer_break_ref} inner_break_ref={&inner_break_ref} />
    }).await.assert_inner_html("<div>wait...(outer)</div>");

    outer_trigger.activate();
    trun.process_events().await.assert_inner_html(
        r#"<div class="content-area"><div class="action-area"><button>Take a break!</button></div><div>wait...(inner)</div></div>"#
    );

    inner_trigger.activate();
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
async fn effects_not_run_when_suspended() {
    #[derive(Properties, Clone)]
    struct Props {
        counter: Rc<RefCell<u64>>,
        trigger: TriggerBus,
        break_ref: NodeRef,
        inc_ref: NodeRef,
    }

    impl PartialEq for Props {
        fn eq(&self, _rhs: &Self) -> bool {
            true
        }
    }

    #[function_component]
    fn Content(props: &Props) -> HtmlResult {
        {
            let counter = props.counter.clone();

            use_effect(move || {
                *counter.borrow_mut() += 1;
                || {}
            });
        }

        let resleep = use_trigger(&props.trigger)?;
        let value = use_state(|| 0);

        let on_increment = {
            let value = value.clone();

            Callback::from(move |_: MouseEvent| value.set(*value + 1))
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
    #[function_component]
    fn App(props: &Props) -> Html {
        let fallback = html! {<div>{"wait..."}</div>};

        html! {
            <Suspense {fallback}>
                <Content ..props.clone() />
            </Suspense>
        }
    }

    let mut trun = TestRunner::new();
    let counter = Rc::new(RefCell::new(0_u64));
    let break_ref = NodeRef::default();
    let inc_ref = NodeRef::default();
    let trigger = TriggerBus::new();

    trun.render(html! {
        <App counter={&counter} break_ref={&break_ref} inc_ref={&inc_ref} trigger={&trigger} />
    })
    .await
    .assert_inner_html("<div>wait...</div>");
    assert_eq!(*counter.borrow(), 0); // effects not called.

    trigger.activate();
    trun.process_events().await.assert_inner_html(
        r#"<div class="content-area"><div class="actual-result">0</div><button>increase</button><div class="action-area"><button>Take a break!</button></div></div>"#
    );
    assert_eq!(*counter.borrow(), 1); // effects ran 1 time.

    inc_ref.cast::<HtmlElement>().unwrap().click();
    trun.process_events().await;
    inc_ref.cast::<HtmlElement>().unwrap().click();
    trun.process_events().await.assert_inner_html(
        r#"<div class="content-area"><div class="actual-result">2</div><button>increase</button><div class="action-area"><button>Take a break!</button></div></div>"#
    );
    assert_eq!(*counter.borrow(), 3); // effects ran 3 times.

    break_ref.cast::<HtmlElement>().unwrap().click();
    trun.process_events()
        .await
        .assert_inner_html("<div>wait...</div>");
    assert_eq!(*counter.borrow(), 3); // effects still ran 3 times.

    trigger.activate();
    trun.process_events().await.assert_inner_html(
        r#"<div class="content-area"><div class="actual-result">2</div><button>increase</button><div class="action-area"><button>Take a break!</button></div></div>"#
    );
    assert_eq!(*counter.borrow(), 4); // effects ran 4 times.
}

#[wasm_bindgen_test]
async fn use_suspending_future_works() {
    #[derive(PartialEq, Properties, Clone)]
    struct AppProps {
        trigger: TriggerBus,
    }
    #[function_component]
    fn Content(AppProps { trigger }: &AppProps) -> HtmlResult {
        let trigger = trigger.clone();
        let _sleep_handle = use_future(|| async move {
            trigger.await;
        })?;

        Ok(html! {
            <div>
                {"Content"}
            </div>
        })
    }
    #[function_component]
    fn App(props: &AppProps) -> Html {
        let fallback = html! { <div>{"wait..."}</div> };

        html! {
            <Suspense {fallback}>
                <Content ..props.clone() />
            </Suspense>
        }
    }

    let mut trun = TestRunner::new();
    let trigger = TriggerBus::new();

    trun.render(html! {
        <App trigger={&trigger} />
    })
    .await
    .assert_inner_html("<div>wait...</div>");

    trigger.activate();
    trun.process_events()
        .await
        .assert_inner_html(r#"<div>Content</div>"#);
}

#[wasm_bindgen_test]
async fn use_suspending_future_with_deps_works() {
    #[derive(PartialEq, Properties)]
    struct ContentProps {
        dep_value: u32,
        trigger: TriggerBus,
    }
    #[function_component(Content)]
    fn content(props: &ContentProps) -> HtmlResult {
        let trigger = props.trigger.clone();
        let delayed_result = use_future_with_deps(
            |dep_value| async move {
                trigger.await;
                dep_value
            },
            props.dep_value,
        )?;

        Ok(html! {
            <div>
                {&*delayed_result}
            </div>
        })
    }
    #[derive(PartialEq, Properties)]
    struct AppProps {
        trigger: TriggerBus,
    }
    #[function_component]
    fn App(AppProps { trigger }: &AppProps) -> Html {
        let fallback = html! {<div>{"wait..."}</div>};

        html! {
            <Suspense {fallback}>
                <Content dep_value={42} {trigger} />
            </Suspense>
        }
    }

    let mut trun = TestRunner::new();
    let trigger = TriggerBus::new();
    trun.render(html! {
        <App trigger={&trigger} />
    })
    .await
    .assert_inner_html("<div>wait...</div>");

    trigger.activate();
    trun.process_events()
        .await
        .assert_inner_html(r#"<div>42</div>"#);
}
