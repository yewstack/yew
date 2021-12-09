use yew::{function_component, html, Properties};

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub best_score: u32,
}

#[function_component(BestScore)]
pub fn score_board_best_score(props: &Props) -> Html {
    html! {
        <div class="best-score">
            <span>{"Highest Record"}</span>
            <h2>{ props.best_score }</h2>
        </div>
    }
}
