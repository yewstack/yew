use yew::prelude::*;
use yew::{function_component, html, Properties};

use crate::components::chessboard_card::ChessboardCard;
use crate::state::{Card, RawCard};

#[derive(Properties, Clone)]
pub struct Props {
    pub cards: Vec<Card>,
    pub on_flip: Callback<RawCard>,
}

impl PartialEq for Props {
    fn eq(&self, other: &Props) -> bool {
        let s_cards = &self.cards;
        let o_cards = &other.cards;
        let s_cards_len = s_cards.len();
        let o_cards_len = o_cards.len();

        match s_cards_len == o_cards_len {
            false => false,
            true => s_cards.iter().all(|c| o_cards.contains(c)),
        }
    }
}

#[function_component(Chessboard)]
pub fn chessboard(props: &Props) -> Html {
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
