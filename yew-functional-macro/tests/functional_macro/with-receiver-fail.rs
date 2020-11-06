use yew::prelude::*;
use yew_functional::functional_component;

#[derive(Clone, Properties, PartialEq)]
struct Props {
    a: usize,
}

struct Test;

impl Test {
    #[functional_component(Comp)]
    fn comp(self, props: &Props) -> Html {
        html! {
            <p>
                { props.a }
            </p>
        }
    }
}

fn main() {}
