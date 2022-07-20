use yew::{function_component, html, Html};

#[function_component]
pub fn Logo() -> Html {
    html! {
        <h1 class="logo">
            <a href="https://examples.yew.rs/function_memory_game" target="_blank">{"Memory"}</a>
        </h1>
    }
}
