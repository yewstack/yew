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

fn compile_fail_missing_lifetime() {
    // missing prop 'a'
    html! { <Comp<Props> /> };
}

fn main() {}
