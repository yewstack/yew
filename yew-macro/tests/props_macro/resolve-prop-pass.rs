use yew::prelude::*;

#[derive(Clone, Properties)]
struct Props {
    n: i32,
}

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

fn compile_pass() {
    yew::props!(Props { n: 1 });
    yew::props!(self::Props { n: 1 });
    yew::props!(MyComp::Properties { n: 2 });
    yew::props!(self::MyComp::Properties { n: 3 });
    yew::props!(<MyComp as Component>::Properties { n: 5 });
}

fn main() {}
