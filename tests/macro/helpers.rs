#[macro_export]
macro_rules! pass_helper {
    ( @html ) => { html! {} };
    ( @html html! { $($view:tt)* }; $($tail:tt)* ) => {
        html! { $($view)* };
        pass_helper! { @ html $($tail)* }
    };
    ( @html $head:stmt; $($tail:tt)* ) => {
        $head
        pass_helper! { @ html $($tail)* }
    };
    ( $($content:tt)* ) => {
        mod test_component;
        use yew::prelude::*;
        #[allow(unused)]
        use test_component::TestComponent;
        struct SubComponent;
        impl Renderable for SubComponent {
            fn render(&self) -> Html {
                pass_helper! { @ html $($content)* }
            }
        }
    };
}
