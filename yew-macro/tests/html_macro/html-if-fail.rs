use yew::prelude::*;

fn compile_fail() {
    html! { if {html!()} };
    let value = 42;
    html! { if value { html!() } };
    html! { if {value} { html!() } };
    let boolean = true;
    html! { if boolean };
    html! { if boolean { () } };

    html! {
        <>
            <div/>
            if value {
            }
        </>
    };
}

fn main() {}
