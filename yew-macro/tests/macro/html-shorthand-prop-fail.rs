use yew::prelude::*;

fn compile_fail() {
    let text = "placeholder-text";
    let length = "20";
    let link = "https://yew.rs/docs/";
    html! { <input placeholder= />};
    html! { <input placeholder= maxlength= />};
    html! { <a href= >{"Yew"}</a>};
}