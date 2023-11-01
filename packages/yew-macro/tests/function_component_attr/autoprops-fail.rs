use yew::prelude::*;

#[autoprops]
#[function_component]
fn CantAcceptReceiver(&self, b: bool) -> Html {
    html! {
        <p>{b}</p>
    }
}

#[autoprops]
fn not_a_function_component(b: bool) -> Html {
    html! {
        <p>{b}</p>
    }
}

#[function_component(WrongAttrsOrder)]
#[autoprops]
fn wrong_attrs_order(b: bool) -> Html {
    html! {
        <p>{b}</p>
    }
}

#[autoprops]
#[function_component(let)]
fn BadFunctionComponent(b: bool) -> Html {
    html! {
        <p>{b}</p>
    }
}

#[derive(PartialEq)]
struct NotClonable(u32);

#[autoprops]
#[function_component]
fn TypeIsNotClone(stuff: NotClonable) -> Html {
    drop(stuff);
    html! {
        <p></p>
    }
}

#[derive(Clone)]
struct NotPartialEq(u32);

#[autoprops]
#[function_component]
fn TypeIsNotPartialEq(stuff: NotPartialEq) -> Html {
    drop(stuff);
    html! {
        <p></p>
    }
}

#[autoprops]
#[function_component]
fn InvalidFieldName(_: u32) -> Html {
    html! {
        <p></p>
    }
}

fn main() {}
