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

fn compile_fail_invalid_type_parameter() {
    // invalid type parameter
    html! { <Comp<INVALID> /> };
}

fn main() {}
