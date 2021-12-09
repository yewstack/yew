use yew::{function_component, html};

#[function_component(Logo)]
pub fn score_board_logo() -> Html {
    html! {
        <h1 class="logo">
            <a href="https://examples.yew.rs/function_memory_game" target="_blank">{"Memory"}</a>
        </h1>
    }
}
