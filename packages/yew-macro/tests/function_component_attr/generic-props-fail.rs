use yew::prelude::*;

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

struct MissingTypeBounds;

fn compile_fail() {
    // missing prop 'a'
    html! { <Comp<Props> /> };

    // invalid type parameter
    html! { <Comp<INVALID> /> };
    // parameter doesn't match bounds
    html! { <Comp<MissingTypeBounds> /> };

    // missing type param
    html! { <Comp /> };
}

fn main() {}
