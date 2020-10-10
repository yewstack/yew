use yew::prelude::*;

fn compile_pass() {
    let boolean = true;
    html! { if boolean { html!() } };
    html! { if boolean { html!() } else { html!() } };
    html! { <div>{if boolean { html!() }}</div> };
    html! { <div>{if boolean { html!() } else { html!() }}</div> };

    let option = Some("text");
    html! { if let Some(text) = option { html!( {text} ) } };
    html! { if let Some(text) = option { html!( {text} ) } else { html!() } };
    html! { <div>{if let Some(text) = option { html!( {text} ) }}</div> };
    html! { <div>{if let Some(text) = option { html!( {text} ) } else { html!() }}</div> };
}

fn main() {}
