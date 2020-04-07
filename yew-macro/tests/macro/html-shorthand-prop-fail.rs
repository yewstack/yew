use yew::prelude::*;

fn compile_fail() {
    let text = "placeholder-text";
    let length = "20";
    let link = "https://yew.rs/docs/";
    html! { <input placeholder= />};
    html! { <input placeholder=text maxlength= />};
    html! { <input placeholder= maxlength=length />};
    html! { <a href= >{"Yew"}</a>};
    html! { <a href=link target= >{"Yew"}</a>};
}