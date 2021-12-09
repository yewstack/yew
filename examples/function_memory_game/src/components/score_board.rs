use yew::{function_component, html, Properties};

use crate::components::{
    score_board_best_score::BestScore, score_board_logo::Logo, score_board_progress::GameProgress,
};

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub unresolved_card_pairs: u8,
    pub best_score: u32,
}

#[function_component(ScoreBoard)]
pub fn score_board(props: &Props) -> Html {
    html! {
        <div class="score-board">
            <Logo />
            <GameProgress unresolved_card_pairs={props.unresolved_card_pairs} />
            <BestScore best_score={props.best_score} />
        </div>
    }
}
