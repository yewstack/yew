use yew::prelude::*;

#[derive(Clone, Properties)]
struct Props {}

struct MyComp;
impl Component for MyComp {
    type Message = ();
    type Properties = Props;

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        unimplemented!()
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        unimplemented!()
    }

    fn view(&self) -> Html {
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
