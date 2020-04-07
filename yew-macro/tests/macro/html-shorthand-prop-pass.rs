use yew::prelude::*;

fn compile_pass() {
    let placeholder = "placeholder-text";
    let maxlength = "20";
    let href = "https://yew.rs/docs/";
    html! { <input placeholder= />};
    html! { <input placeholder= maxlength= />};
    html! { <a href= >{"Yew"}</a>};
}

fn main() {}