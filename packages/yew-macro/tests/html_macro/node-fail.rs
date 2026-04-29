#![allow(uncommon_codepoints)]
use yew::prelude::*;

fn compile_fail() {
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

    html! { <a 𐊖𐊗𐊒𐊓="value"></a> };
}

fn main() {}
