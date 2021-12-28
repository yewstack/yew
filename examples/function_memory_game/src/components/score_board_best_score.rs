use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub best_score: u32,
}

#[function_component]
pub fn ScoreBoardBestScore(props: &Props) -> Html {
    html! {
        <div class="best-score">
            <span>{"Highest Record"}</span>
            <h2>{ props.best_score }</h2>
        </div>
    }
}
