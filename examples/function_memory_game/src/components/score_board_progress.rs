use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub unresolved_card_pairs: u8,
}

#[function_component]
pub fn ScoreBoardGameProgress(props: &Props) -> Html {
    html! {
        <div class="game-progress">
            <span>{"Cards not Matched"}</span>
            <h2>{ props.unresolved_card_pairs }</h2>
        </div>
    }
}
