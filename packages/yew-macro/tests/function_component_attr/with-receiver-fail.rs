use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
struct Props {
    a: usize,
}

struct Test;

impl Test {
    #[function_component(Comp)]
    fn comp(self, props: &Props) -> Html {
        html! {
            <p>
                { props.a }
            </p>
        }
    }
}

fn main() {}
