use yew::prelude::*;

use crate::components::{
    score_board_best_score::ScoreBoardBestScore, score_board_logo::ScoreBoardLogo,
    score_board_progress::ScoreBoardGameProgress,
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
            <ScoreBoardLogo />
            <ScoreBoardGameProgress unresolved_card_pairs={props.unresolved_card_pairs} />
            <ScoreBoardBestScore best_score={props.best_score} />
        </div>
    }
}
