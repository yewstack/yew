use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
struct Props {
    a: usize,
}

#[component(Comp1)]
fn comp_1(_props: &Props) {}

#[component(Comp)]
fn comp(_props: &Props) -> u32 {
    1
}

fn main() {}
