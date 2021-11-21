use yew::prelude::*;

#[derive(Clone, Properties)]
struct Props {}

struct MyComp;
impl Component for MyComp {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        unimplemented!()
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        unimplemented!()
    }
}

trait NotAComponent {
    type Properties;
}

struct MyNotAComponent;
impl NotAComponent for MyNotAComponent {
    type Properties = ();
}

fn compile_fail() {
    yew::props!(Vec<_> {});
    yew::props!(MyComp {});
    yew::props!(MyNotAComponent::Properties {});
}

fn main() {}
