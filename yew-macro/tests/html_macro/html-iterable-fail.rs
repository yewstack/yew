use yew::prelude::*;

fn compile_fail() {
    html! { for };
    html! { for () };
    html! { for {()} };
    html! { for Vec::<()>::new().into_iter() };

    let empty = Vec::<()>::new().into_iter();
    html! { for empty };

    let empty = Vec::<()>::new();
    html! { for empty.iter() };

    html! {
        <>
            <div/>
            { for () }
        </>
    };

    // new syntax
    html! { <div>for {}</div> };
    html! { <div>for {()}</div> };
    html! { <div>for {Vec::<()>::new().into_iter()}</div> };

    let empty = Vec::<()>::new().into_iter();
    html! { <div>for {empty}</div> };

    let empty = Vec::<()>::new();
    html! { <div>for {empty.iter()}</div> };

    html! {
        <>
            <div/>
            { for {()} }
        </>
    };
}

fn main() {}
