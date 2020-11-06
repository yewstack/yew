use yew::prelude::*;
use yew_functional::functional_component;

#[derive(Clone, Properties, PartialEq)]
struct Props {
    a: usize,
}

#[functional_component(Comp)]
struct Test;

fn main() {}
