use yew::prelude::*;

#[function_component]
fn TestComponent() -> Html {
    html! {
        <svg>
            <defs>
                <filter id="glow">
                    <feDropShadow dx="0" dy="0" stdDeviation="10" flood-color="red"/>
                    <feGaussianBlur stdDeviation="2.5"/>
                    <feColorMatrix type="matrix"/>
                </filter>
            </defs>
            <rect width="100" height="100" filter="url(#glow)" />
        </svg>
    }
}

fn main() {
    println!("Test compiled successfully!");
}