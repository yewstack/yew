use yew::{function_component, html, Html, Properties};

#[derive(PartialEq, Eq, Properties, Clone)]
pub struct Props {
    pub best_score: u32,
}

#[function_component]
pub fn BestScore(props: &Props) -> Html {
    html! {
        <div class="best-score">
            <span>{"Highest Record"}</span>
            <h2>{ props.best_score }</h2>
        </div>
    }
}
