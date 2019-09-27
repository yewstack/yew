use yew::prelude::*;

fn compile_fail() {
    html! { "valid" "invalid" };
    html! { <span>{ "valid" "invalid" }</span> };
    html! { () };
    html! { invalid };

    // unsupported literals
    html! {  b'a' };
    html! {  b"str" };
    html! {  1111111111111111111111111111111111111111111111111111111111111111111111111111 };
    html! {  <span>{ b'a' }</span> };
    html! {  <span>{ b"str" }</span> };
    html! {  <span>{ 1111111111111111111111111111111111111111111111111111111111111111111111111111 }</span> };

    let not_node = || ();
    html! {
        not_node()
    };
}

fn main() {}
