use yew::prelude::*;

fn compile_fail() {
    html! { "valid" "invalid" };
    html! { <span>{ "valid" "invalid" }</span> };
    html! { () };
    html! { invalid };

    // unsupported literals
    html! {  b'a' };
    html! {  b"str" };
    html! {  <span>{ b'a' }</span> };
    html! {  <span>{ b"str" }</span> };

    let not_node = || ();
    html! {
        not_node()
    };
}

fn main() {}
