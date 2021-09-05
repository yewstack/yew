#![no_implicit_prelude]

#[derive(::std::clone::Clone, ::yew::Properties, ::std::cmp::PartialEq)]
struct Props {
    n: i32,
}

struct MyComp;
impl ::yew::Component for MyComp {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &::yew::Context<Self>) -> Self {
        ::std::unimplemented!()
    }
    fn view(&self, _ctx: &::yew::Context<Self>) -> ::yew::Html {
        ::std::unimplemented!()
    }
}

fn compile_pass() {
    ::yew::props!(Props { n: 1 });
    ::yew::props!(self::Props { n: 1 });
    ::yew::props!(MyComp::Properties { n: 2 });
    ::yew::props!(self::MyComp::Properties { n: 3 });
    ::yew::props!(<MyComp as ::yew::Component>::Properties { n: 5});
}

fn main() {}
