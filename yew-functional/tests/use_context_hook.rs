extern crate wasm_bindgen_test;

use std::rc::Rc;
use wasm_bindgen_test::*;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

extern crate yew;

use yew::{html, App, Html, Properties};
use yew_functional::{
    use_context, use_effect, use_ref, use_state, ContextProvider, FunctionComponent,
    FunctionProvider,
};

fn obtain_result(id: &str) -> String {
    return yew::utils::document()
        .get_element_by_id(id)
        .expect("No result found. Most likely, the application crashed and burned")
        .inner_html();
}

#[wasm_bindgen_test]
fn use_context_scoping_works() {
    #[derive(Clone, Debug)]
    struct ExampleContext(String);
    struct UseContextFunctionOuter {}
    struct UseContextFunctionInner {}
    struct ExpectNoContextFunction {}
    type UseContextComponent = FunctionComponent<UseContextFunctionOuter>;
    type UseContextComponentInner = FunctionComponent<UseContextFunctionInner>;
    type ExpectNoContextComponent = FunctionComponent<ExpectNoContextFunction>;
    impl FunctionProvider for ExpectNoContextFunction {
        type TProps = ();

        fn run(_props: &Self::TProps) -> Html {
            if use_context::<ExampleContext>().is_some() {
                yew::services::ConsoleService::new().log(&format!(
                    "Context should be None here, but was {:?}!",
                    use_context::<ExampleContext>().unwrap()
                ));
            };
            return html! {
                <div></div>
            };
        }
    }
    impl FunctionProvider for UseContextFunctionOuter {
        type TProps = ();

        fn run(_props: &Self::TProps) -> Html {
            type ExampleContextProvider = ContextProvider<ExampleContext>;
            return html! {
                <div>
                    <ExampleContextProvider context=ExampleContext("wrong1".into())>
                        <div>{"ignored"}</div>
                    </ExampleContextProvider>
                    <ExampleContextProvider context=ExampleContext("wrong2".into())>
                        <ExampleContextProvider context=ExampleContext("correct".into())>
                            <ExampleContextProvider context=ExampleContext("wrong1".into())>
                                <div>{"ignored"}</div>
                            </ExampleContextProvider>
                            <UseContextComponentInner />
                        </ExampleContextProvider>
                    </ExampleContextProvider>
                    <ExampleContextProvider context=ExampleContext("wrong3".into())>
                        <div>{"ignored"}</div>
                    </ExampleContextProvider>
                    <ExpectNoContextComponent />
                </div>
            };
        }
    }
    impl FunctionProvider for UseContextFunctionInner {
        type TProps = ();

        fn run(_props: &Self::TProps) -> Html {
            let context = use_context::<ExampleContext>();
            return html! {
                <div id="result">{ &context.unwrap().0 }</div>
            };
        }
    }

    let app: App<UseContextComponent> = yew::App::new();
    app.mount(yew::utils::document().get_element_by_id("output").unwrap());
    let result: String = obtain_result("result");
    assert_eq!("correct", result);
}

#[wasm_bindgen_test]
fn use_context_works_with_multiple_types() {
    #[derive(Clone, Debug, PartialEq)]
    struct ContextA(u32);
    #[derive(Clone, Debug, PartialEq)]
    struct ContextB(u32);

    struct Test1Function;
    impl FunctionProvider for Test1Function {
        type TProps = ();

        fn run(_props: &Self::TProps) -> Html {
            assert_eq!(use_context::<ContextA>(), Some(Rc::new(ContextA(2))));
            assert_eq!(use_context::<ContextB>(), Some(Rc::new(ContextB(1))));

            return html! {};
        }
    }
    type Test1 = FunctionComponent<Test1Function>;

    struct Test2Function;
    impl FunctionProvider for Test2Function {
        type TProps = ();

        fn run(_props: &Self::TProps) -> Html {
            assert_eq!(use_context::<ContextA>(), Some(Rc::new(ContextA(0))));
            assert_eq!(use_context::<ContextB>(), Some(Rc::new(ContextB(1))));

            return html! {};
        }
    }
    type Test2 = FunctionComponent<Test2Function>;

    struct Test3Function;
    impl FunctionProvider for Test3Function {
        type TProps = ();

        fn run(_props: &Self::TProps) -> Html {
            assert_eq!(use_context::<ContextA>(), Some(Rc::new(ContextA(0))));
            assert_eq!(use_context::<ContextB>(), None);

            return html! {};
        }
    }
    type Test3 = FunctionComponent<Test3Function>;

    struct Test4Function;
    impl FunctionProvider for Test4Function {
        type TProps = ();

        fn run(_props: &Self::TProps) -> Html {
            assert_eq!(use_context::<ContextA>(), None);
            assert_eq!(use_context::<ContextB>(), None);

            return html! {};
        }
    }
    type Test4 = FunctionComponent<Test4Function>;

    struct TestFunction;
    impl FunctionProvider for TestFunction {
        type TProps = ();

        fn run(_props: &Self::TProps) -> Html {
            type ContextAProvider = ContextProvider<ContextA>;
            type ContextBProvider = ContextProvider<ContextB>;

            return html! {
                <div>
                    <ContextAProvider context=ContextA(0)>
                        <ContextBProvider context=ContextB(1)>
                            <ContextAProvider context=ContextA(2)>
                                <Test1/>
                            </ContextAProvider>
                            <Test2/>
                        </ContextBProvider>
                        <Test3/>
                    </ContextAProvider>
                    <Test4 />
                </div>
            };
        }
    }
    type TestComponent = FunctionComponent<TestFunction>;

    let app: App<TestComponent> = yew::App::new();
    app.mount(yew::utils::document().get_element_by_id("output").unwrap());
}

#[wasm_bindgen_test]
fn use_context_update_works() {
    #[derive(Clone, Debug)]
    struct MyContext(String);

    #[derive(Clone, Debug, PartialEq, Properties)]
    struct ContextOutletProps {
        id: String,
    }

    struct ContextOutletFunction;
    impl FunctionProvider for ContextOutletFunction {
        type TProps = ContextOutletProps;

        fn run(props: &Self::TProps) -> Html {
            let counter = use_ref(|| 0);
            *counter.borrow_mut() += 1;

            let ctx = use_context::<Rc<MyContext>>().expect("context not passed down");

            return html! {
                <div id=props.id.clone()>
                    { format!("current: {}\n", ctx.0) }
                    { format!("total: {}\n", counter.borrow()) }
                </div>
            };
        }
    }
    type ContextOutlet = FunctionComponent<ContextOutletFunction>;

    struct TestFunction;
    impl FunctionProvider for TestFunction {
        type TProps = ();

        fn run(_props: &Self::TProps) -> Html {
            type MyContextProvider = ContextProvider<Rc<MyContext>>;

            let (ctx, set_ctx) = use_state(|| MyContext("hello".into()));
            let rendered = use_ref(|| false);

            use_effect(move || {
                if !*rendered.borrow() {
                    set_ctx(MyContext("world".into()));
                    *rendered.borrow_mut() = true;
                }
                || {}
            });

            return html! {
                <>
                    <MyContextProvider context=ctx>
                        <div>
                            <ContextOutlet id="test-nested"/>
                        </div>

                        <ContextOutlet id="test-child"/>
                    </MyContextProvider>
                </>
            };
        }
    }
    type TestComponent = FunctionComponent<TestFunction>;

    let app: App<TestComponent> = yew::App::new();
    app.mount(
        yew::utils::document()
            .get_element_by_id("output")
            .unwrap()
            .clone(),
    );

    assert_eq!(obtain_result("test-nested"), "current: world\ntotal: 2\n");
    assert_eq!(obtain_result("test-child"), "current: world\ntotal: 2\n");
}
