use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
struct Props {
    a: usize,
}

#[function_component]
fn Comp(props: &Props) -> Html {
    html! {
        <p>
            { props.a }
        </p>
    }
}

fn main() {
    let _ = html! {
        <Comp a={0} />
    };
}
