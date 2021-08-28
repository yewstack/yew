use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
struct Props {
    n: i32,
}

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

fn compile_pass() {
    yew::props!(Props { n: 1 });
    yew::props!(self::Props { n: 1 });
    yew::props!(MyComp::Properties { n: 2 });
    yew::props!(self::MyComp::Properties { n: 3 });
    yew::props!(<MyComp as Component>::Properties { n: 5 });
}

fn main() {}
