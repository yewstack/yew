#[macro_export]
macro_rules! pass_helper {
    ( @html ) => { html! {} };
    ( @html html! { $($view:tt)* }; $($tail:tt)* ) => {
        let _: Html<Self> = html! { $($view)* };
        pass_helper! { @ html $($tail)* }
    };
    ( @html $head:stmt; $($tail:tt)* ) => {
        $head;
        pass_helper! { @ html $($tail)* }
    };
    ( $($content:tt)* ) => {
        mod test_component;
        use test_component::TestComponent;
        use yew::prelude::*;
        impl Renderable<TestComponent> for TestComponent {
            fn view(&self) -> Html<Self> {
                pass_helper! { @ html $($content)* }
            }
        }
    };
}
