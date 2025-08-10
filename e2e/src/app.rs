use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    html! {
    <svg>
        <defs>
            <filter id="glow">
                <feDropShadow dx="0" dy="0" stdDeviation="10" flood-color="red"/>
            </filter>
        </defs>
        <rect width="100" height="100" filter="url(#glow)" />
    </svg>
    }
}
