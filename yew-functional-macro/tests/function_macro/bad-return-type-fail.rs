use yew::prelude::*;
use yew_functional::function_component;

#[derive(Clone, Properties, PartialEq)]
struct Props {
    a: usize,
}

#[function_component(Comp1)]
fn comp_1(_props: &Props) {}

#[function_component(Comp)]
fn comp(_props: &Props) -> u32 {
    1
}

fn main() {}
