#[macro_use]
mod helpers;

pass_helper! {
    html! {};
    html! { <></> };
    html! {
        <>
            <></>
            <></>
        </>
    };
}

fn main() {}
