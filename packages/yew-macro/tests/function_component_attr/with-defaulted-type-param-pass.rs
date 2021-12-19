use yew::prelude::*;

#[derive(Properties, Debug)]
pub struct CompProps<A> {
    #[prop_or_default]
    _phantom: std::marker::PhantomData<A>,
}

impl<A> PartialEq for CompProps<A> {
    fn eq(&self, _rhs: &Self) -> bool {
        true
    }
}

#[function_component(Comp)]
pub fn comp<A = ()>(_props: &CompProps<A>) -> Html {
    todo!()
}

#[function_component(App)]
pub fn app() -> Html {
    html! { <Comp /> } // No generics here.
}

fn main() {}
