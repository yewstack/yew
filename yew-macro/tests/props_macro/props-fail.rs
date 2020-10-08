use yew::prelude::*;

#[derive(Clone, Properties)]
struct Props {
    a: usize,
}

fn compile_fail() {
    yew::props!(Props with ());
    yew::props!(Props ref=NodeRef::default() key="key");
    yew::props!(Props a=5 fail=10)
}

fn main() {}
