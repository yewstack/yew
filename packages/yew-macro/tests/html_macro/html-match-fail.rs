use yew::prelude::*;

fn main() {
    // Missing scrutinee
    html! { match {} };

    // Missing body
    html! { match 42 };

    // Empty match (no arms)
    html! { match 42 {} };

    // `break` followed by HTML in an unbraced arm. Rust doesn't accept `<span/>`
    // as an expression so it can't be a break value; user must wrap in braces.
    let _ = html! {
        for _ in 0..1 {
            match () {
                () => break <span/>,
            }
        }
    };

    // Same pattern with `continue`.
    let _ = html! {
        for _ in 0..1 {
            match () {
                () => continue <div/>,
            }
        }
    };

    // Same pattern with `return`.
    fn return_html() -> Html {
        html! {
            for _ in 0..1 {
                match () {
                    () => return <p/>,
                }
            }
        }
    }
    let _ = return_html;

    // Final arm without trailing comma.
    let _ = html! {
        for _ in 0..1 {
            match () {
                () => break <div/>
            }
        }
    };
}
