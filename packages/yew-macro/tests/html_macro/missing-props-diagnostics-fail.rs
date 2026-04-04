use yew::prelude::*;

#[component]
pub fn App() -> Html {
    html! {
        <Foo />
    }
}

#[component]
pub fn App1() -> Html {
    html! {
        <Foo bar={"bar".to_string()} />
    }
}

#[component]
pub fn App2() -> Html {
    html! {
        <Foo bar={"bar".to_string()} baz={42} />
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct FooProps {
    pub bar: String,
    pub baz: u32,
}

#[component]
pub fn Foo(_props: &FooProps) -> Html {
    html! {}
}

fn main() {}