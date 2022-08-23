#![cfg(target_arch = "wasm32")]

mod common;

use std::rc::Rc;
use std::time::Duration;

use common::obtain_result_by_id;
use wasm_bindgen_test::*;
use yew::platform::time::sleep;
use yew::prelude::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn use_context_scoping_works() {
    #[derive(Clone, Debug, PartialEq)]
    struct ExampleContext(String);

    #[function_component]
    fn ExpectNoContextComponent() -> Html {
        let example_context = use_context::<ExampleContext>();

        if example_context.is_some() {
            console_log!(
                "Context should be None here, but was {:?}!",
                example_context
            );
        };
        html! {
            <div></div>
        }
    }

    #[function_component]
    fn UseContextComponent() -> Html {
        type ExampleContextProvider = ContextProvider<ExampleContext>;
        html! {
            <div>
                <ExampleContextProvider context={ExampleContext("wrong1".into())}>
                    <div>{"ignored"}</div>
                </ExampleContextProvider>
                <ExampleContextProvider context={ExampleContext("wrong2".into())}>
                    <ExampleContextProvider context={ExampleContext("correct".into())}>
                        <ExampleContextProvider context={ExampleContext("wrong1".into())}>
                            <div>{"ignored"}</div>
                        </ExampleContextProvider>
                        <UseContextComponentInner />
                    </ExampleContextProvider>
                </ExampleContextProvider>
                <ExampleContextProvider context={ExampleContext("wrong3".into())}>
                    <div>{"ignored"}</div>
                </ExampleContextProvider>
                <ExpectNoContextComponent />
            </div>
        }
    }

    #[function_component]
    fn UseContextComponentInner() -> Html {
        let context = use_context::<ExampleContext>();
        html! {
            <div id="result">{ &context.unwrap().0 }</div>
        }
    }

    yew::Renderer::<UseContextComponent>::with_root(
        gloo::utils::document().get_element_by_id("output").unwrap(),
    )
    .render();

    sleep(Duration::ZERO).await;

    let result: String = obtain_result_by_id("result");
    assert_eq!("correct", result);
}

#[wasm_bindgen_test]
async fn use_context_works_with_multiple_types() {
    #[derive(Clone, Debug, PartialEq)]
    struct ContextA(u32);
    #[derive(Clone, Debug, PartialEq)]
    struct ContextB(u32);

    #[function_component]
    fn Test1() -> Html {
        let ctx_a = use_context::<ContextA>();
        let ctx_b = use_context::<ContextB>();

        assert_eq!(ctx_a, Some(ContextA(2)));
        assert_eq!(ctx_b, Some(ContextB(1)));

        html! {}
    }

    #[function_component]
    fn Test2() -> Html {
        let ctx_a = use_context::<ContextA>();
        let ctx_b = use_context::<ContextB>();

        assert_eq!(ctx_a, Some(ContextA(0)));
        assert_eq!(ctx_b, Some(ContextB(1)));

        html! {}
    }

    #[function_component]
    fn Test3() -> Html {
        let ctx_a = use_context::<ContextA>();
        let ctx_b = use_context::<ContextB>();

        assert_eq!(ctx_a, Some(ContextA(0)));
        assert_eq!(ctx_b, None);

        html! {}
    }

    #[function_component]
    fn Test4() -> Html {
        let ctx_a = use_context::<ContextA>();
        let ctx_b = use_context::<ContextB>();

        assert_eq!(ctx_a, None);
        assert_eq!(ctx_b, None);

        html! {}
    }

    #[function_component]
    fn TestComponent() -> Html {
        type ContextAProvider = ContextProvider<ContextA>;
        type ContextBProvider = ContextProvider<ContextB>;

        html! {
            <div>
                <ContextAProvider context={ContextA(0)}>
                    <ContextBProvider context={ContextB(1)}>
                        <ContextAProvider context={ContextA(2)}>
                            <Test1/>
                        </ContextAProvider>
                        <Test2/>
                    </ContextBProvider>
                    <Test3/>
                </ContextAProvider>
                <Test4 />
            </div>
        }
    }

    yew::Renderer::<TestComponent>::with_root(
        gloo::utils::document().get_element_by_id("output").unwrap(),
    )
    .render();

    sleep(Duration::ZERO).await;
}

#[wasm_bindgen_test]
async fn use_context_update_works() {
    #[derive(Clone, Debug, PartialEq)]
    struct MyContext(String);

    #[derive(Clone, Debug, PartialEq, Properties)]
    struct RenderCounterProps {
        id: String,
        children: Children,
    }

    #[function_component]
    fn RenderCounter(props: &RenderCounterProps) -> Html {
        let counter = use_mut_ref(|| 0);
        *counter.borrow_mut() += 1;
        html! {
            <>
                <div id={props.id.clone()}>
                    { format!("total: {}", counter.borrow()) }
                </div>
                { props.children.clone() }
            </>
        }
    }

    #[derive(Clone, Debug, PartialEq, Properties)]
    struct ContextOutletProps {
        id: String,
        #[prop_or_default]
        magic: usize,
    }

    #[function_component]
    fn ContextOutlet(props: &ContextOutletProps) -> Html {
        let counter = use_mut_ref(|| 0);
        *counter.borrow_mut() += 1;

        let ctx = use_context::<Rc<MyContext>>().expect("context not passed down");

        html! {
            <>
                <div>{ format!("magic: {}\n", props.magic) }</div>
                <div id={props.id.clone()}>
                    { format!("current: {}, total: {}", ctx.0, counter.borrow()) }
                </div>
            </>
        }
    }

    #[function_component]
    fn TestComponent() -> Html {
        type MyContextProvider = ContextProvider<Rc<MyContext>>;

        let ctx = use_state(|| MyContext("hello".into()));
        let rendered = use_mut_ref(|| 0);

        // this is used to force an update specific to test-2
        let magic_rc = use_state(|| 0);
        let magic: usize = *magic_rc;
        {
            let ctx = ctx.clone();
            use_effect(move || {
                let count = *rendered.borrow();
                match count {
                    0 => {
                        ctx.set(MyContext("world".into()));
                        *rendered.borrow_mut() += 1;
                    }
                    1 => {
                        // force test-2 to re-render.
                        magic_rc.set(1);
                        *rendered.borrow_mut() += 1;
                    }
                    2 => {
                        ctx.set(MyContext("hello world!".into()));
                        *rendered.borrow_mut() += 1;
                    }
                    _ => (),
                };
                || {}
            });
        }
        html! {
            <MyContextProvider context={Rc::new((*ctx).clone())}>
                <RenderCounter id="test-0">
                    <ContextOutlet id="test-1"/>
                    <ContextOutlet id="test-2" {magic}/>
                </RenderCounter>
            </MyContextProvider>
        }
    }

    yew::Renderer::<TestComponent>::with_root(
        gloo::utils::document().get_element_by_id("output").unwrap(),
    )
    .render();

    sleep(Duration::ZERO).await;

    // 1 initial render + 3 update steps
    assert_eq!(obtain_result_by_id("test-0"), "total: 4");

    // 1 initial + 2 context update
    assert_eq!(
        obtain_result_by_id("test-1"),
        "current: hello world!, total: 3"
    );

    // 1 initial + 1 context update + 1 magic update + 1 context update
    assert_eq!(
        obtain_result_by_id("test-2"),
        "current: hello world!, total: 4"
    );
}
