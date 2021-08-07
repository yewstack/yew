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
}

fn main() {}
