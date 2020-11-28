use yew::prelude::*;
use std::iter;

fn compile_pass() {
    html! { for iter::empty::<Html>() };
    html! { for Vec::<Html>::new() };
    html! { for Vec::<Html>::new().into_iter() };
    html! { for (0..3).map(|num| { html! { <span>{num}</span> } }) };
    html! { for {iter::empty::<Html>()} };
    let empty: Vec<Html> = Vec::new();
    html! { for empty.into_iter() };
    let empty: Vec<Html> = Vec::new();
    html! { for empty };

    // test as child
    html! { <div>{for iter::empty::<Html>()}</div> };
    html! { <div>{for Vec::<Html>::new()}</div> };
    html! { <div>{for Vec::<Html>::new().into_iter()}</div> };
    html! { <div>{for (0..3).map(|num| { html! { <span>{num}</span> } })}</div> };
    html! { <div>{for {iter::empty::<Html>()}}</div> };
    let empty: Vec<Html> = Vec::new();
    html! { <div>{for empty.into_iter()}</div> };
    let empty: Vec<Html> = Vec::new();
    html! { <div>{for empty}</div> };

    // new syntax
    html! { <div>for {iter::empty::<Html>()}</div> };
    html! { <div>for {Vec::<Html>::new()}</div> };
    html! { <div>for {Vec::<Html>::new().into_iter()}</div> };
    html! { <div>for {(0..3).map(|num| { html! { <span>{num}</span> } })}</div> };
    let empty: Vec<Html> = Vec::new();
    html! { <div>for {empty.into_iter()}</div> };
    let empty: Vec<Html> = Vec::new();
    html! { <div>for {empty}</div> };
}

fn main() {}
