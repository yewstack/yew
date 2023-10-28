use yew::prelude::*;
use yew::{component, html, Properties};

use crate::components::chessboard_card::ChessboardCard;
use crate::state::{Card, RawCard};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub cards: Vec<Card>,
    pub on_flip: Callback<RawCard>,
}
#[component]
pub fn Chessboard(props: &Props) -> Html {
    html! {
        <div class="chess-board">
        { for props.cards.iter().map(|card|
            html! {
                <ChessboardCard card={card.clone()} on_flip={&props.on_flip} />
            }
        ) }
        </div>
    }
}
