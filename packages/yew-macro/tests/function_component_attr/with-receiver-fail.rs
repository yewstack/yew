use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
struct Props {
    a: usize,
}

struct Test;

impl Test {
    #[component(Comp)]
    fn comp(self, props: &Props) -> Html {
        html! {
            <p>
                { props.a }
            </p>
        }
    }
}

fn main() {}
