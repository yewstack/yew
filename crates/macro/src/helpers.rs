#[macro_export]
macro_rules! test_html_block {
    ( |$tc:ident| $($view:tt)* ) => {
        test_html! { @ gen $tc { $($view)* } }
    };
}

#[macro_export]
macro_rules! test_html {
    ( |$tc:ident| $($view:tt)* ) => {
        test_html! { @ gen $tc { html! { $($view)* } } }
    };
    ( @gen $tc:ident $view:block ) => {
        mod $tc {
            use ::yew::prelude::*;
            use ::yew_macro::html;
            use super::*;

            struct TestComponent {}
            impl Component for TestComponent {
                type Message = ();
                type Properties = ();

                fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
                    TestComponent {}
                }

                fn update(&mut self, _: Self::Message) -> ShouldRender {
                    true
                }
            }

            impl Renderable<TestComponent> for TestComponent {
                fn view(&self) -> Html<Self> {
                    $view
                }
            }
        }
    };
}
