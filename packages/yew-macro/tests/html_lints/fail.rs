use yew::prelude::*;

fn main() {
    let bad_a = html! {
        <a>{ "I don't have a href attribute" }</a>
    };
    let still_bad_a = html! {
        <a href="javascript:void(0)">{ "I have a malformed href attribute" }</a>
    };
    let bad_img = html! {
        <img src="img.jpeg"/>
    };
    compile_error!("");
}
