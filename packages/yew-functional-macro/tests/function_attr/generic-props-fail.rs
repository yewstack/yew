use yew::prelude::*;
use yew_functional::function_component;

#[derive(Clone, Properties, PartialEq)]
struct Props {
    a: usize,
}

#[function_component(Comp)]
fn comp<P: Properties + PartialEq>(_props: &P) -> Html {
    html! {
        <p></p>
    }
}

fn compile_fail() {
    // missing prop 'a'
    html! { <Comp<Props> /> };
    
    // invalid type parameter
    html! { <Comp<INVALID> /> };
    // parameter doesn't match bounds
    html! { <Comp<()> /> };

    // missing type param
    html! { <Comp /> };
}


fn main() {}
