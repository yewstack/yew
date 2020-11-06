use yew::prelude::*;
use yew_functional::functional_component;

#[derive(Clone, Properties, PartialEq)]
struct Props {
    a: usize,
}

#[functional_component(Comp1)]
fn comp_1(_props: &Props) {}

#[functional_component(Comp)]
fn comp(_props: &Props) -> u32 {
    1
}

fn main() {}
