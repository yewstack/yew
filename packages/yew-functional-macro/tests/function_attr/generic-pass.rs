use yew::prelude::*;
use yew_functional::function_component;

#[derive(Clone, Properties, PartialEq)]
struct Props {
    a: usize,
}

#[function_component(Comp)]
fn comp<P>(_props: &P) -> Html
where
    P: Properties + PartialEq,
{
    html! {
        <p></p>
    }
}

#[function_component(Comp2)]
fn comp2<P: Properties + PartialEq>(_props: &P) -> Html {
    html! {
        <p></p>
    }
}

#[function_component(Comp3)]
fn comp3<T1, T2>(_props: &()) -> Html {
    html! {
        <p></p>
    }
}

fn main() {}
