use yew::prelude::*;

fn main() {
    // Missing scrutinee
    html! { match {} };

    // Missing body
    html! { match 42 };

    // Empty match (no arms)
    html! { match 42 {} };

    // html! in unbraced match arm
    html! {
        match 1 {
            1 => html! { <h1>{"Hello"}</h1> },
            _ => <h1>{"Goodbye"}</h1>,
        }
    };

    // html! in braced match arm with let binding
    html! {
        match 1 {
            1 => { let a = 1; html! { <h1>{a}</h1> } },
            _ => <h1>{"Goodbye"}</h1>,
        }
    };

    // Unnecessary fragment in braced match arm
    html! {
        match 1 {
            1 => { <><h1>{"Hello"}</h1><p>{"World"}</p></> }
            _ => <h1>{"Goodbye"}</h1>,
        }
    };
}
