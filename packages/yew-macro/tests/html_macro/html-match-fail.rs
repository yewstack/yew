use yew::prelude::*;

fn main() {
    // Missing scrutinee
    html! { match {} };

    // Missing body
    html! { match 42 };

    // Empty match (no arms)
    html! { match 42 {} };
}
