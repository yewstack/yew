use yew::prelude::*;

fn compile_pass() {
    /*
    html! { for iter::empty::<Html>() };
    html! { for Vec::<Html>::new() };
    html! { for Vec::<Html>::new().into_iter() };
    html! { for (0..3).map(|num| { html! { <span>{num}</span> } }) };
    html! { for {iter::empty::<Html>()} };

    let empty: Vec<Html> = Vec::new();
    html! { for empty.into_iter() };

    let empty: Vec<Html> = Vec::new();
    html! { for empty };
    */

    let boolean = true;
    html! { if boolean { html!() } };
    html! { if boolean { html!() } else { html!() } };

    let option = Some("text");
    html! { if let Some(text) = option { html!( {text} ) } };
    html! { if let Some(text) = option { html!( {text} ) } else { html!() } };
}

fn main() {}
