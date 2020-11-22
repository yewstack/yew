use yew::prelude::*;

#[derive(PartialEq, Properties)]
struct Props {
    n: i32,
}

struct MyComp;
impl Component for MyComp {
    type Message = ();
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        unimplemented!()
    }

    fn view(&self, _: &Context<Self>) -> Html {
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
