use yew::prelude::*;

fn compile_pass() {
    html! { if true { <div/> } };
    html! { if true { html!() } };
    html! { if true {} };
    html! { if true { let _x = 42; html!() } };
    html! { if true { <div/> } else { html!() } };
    html! { if false { <div/> } else if true { html!() } else { html!() } };
    html! { if false { <div/> } else if false { html!() } else {} };
    /*

    let option = Some("text");
    html! { if let Some(text) = option { html!( {text} ) } };
    html! { if let Some(text) = option { html!( {text} ) } else { html!() } };
    html! { <div>if let Some(text) = option { html!( {text} ) }</div> };
    html! { <div>if let Some(text) = option { html!( {text} ) } else { html!() }</div> };
    */
}

fn main() {}
