use yew::prelude::*;

#[derive(Clone, Properties)]
struct Props {
    a: usize,
    #[prop_or_default]
    b: usize,
}

fn compile_pass() {
    yew::props!(Props { a: 5 });
    let (a, b) = (3, 5);
    yew::props!(Props { a, b });
}

fn main() {}
