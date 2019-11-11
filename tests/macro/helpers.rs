#[macro_export]
macro_rules! pass_helper {
    ( @html ) => { html! {} };
    ( @html html! { $($view:tt)* }; $($tail:tt)* ) => {
        let _: Html<TestComponent> = html! { $($view)* };
        pass_helper! { @ html $($tail)* }
    };
    ( @html $head:stmt; $($tail:tt)* ) => {
        $head
        pass_helper! { @ html $($tail)* }
    };
    ( $($content:tt)* ) => {
        mod test_component;
        use test_component::TestComponent;
        use yew::prelude::*;
        struct SubComponent;
        impl Renderable<TestComponent> for SubComponent {
            fn render(&self) -> Html<TestComponent> {
                pass_helper! { @ html $($content)* }
            }
        }
    };
}
