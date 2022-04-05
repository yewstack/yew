use yew::prelude::*;

fn main() {
    let bad_a = html! {
        <a>{ "I don't have a href attribute" }</a>
    };
    let bad_a_2 = html! {
        <a href="#">{ "I have a malformed href attribute" }</a>
    };
    let bad_a_3 = html! {
        <a href="javascript:void(0)">{ "I have a malformed href attribute" }</a>
    };
    let bad_img = html! {
        <img src="img.jpeg"/>
    };
    let misformed_tagname = html! {
        <tExTAreA />
    };
    compile_error!("This macro call exists to deliberately fail the compilation of the test so we can verify output of lints");
}
