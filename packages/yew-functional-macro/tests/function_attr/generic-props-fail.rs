use yew::prelude::*;
use yew_functional::function_component;

#[derive(Clone, Properties, PartialEq)]
struct Props {
    a: usize,
}

// TODO: improve error message
#[function_component(Comp)]
fn comp<P: Properties + PartialEq>(_props: &P) -> Html {
    html! {
        <p></p>
    }
}

fn compile_pass() {
    html! { <Comp<Props> /> }; // missing prop 'a'
}

fn main() {}
