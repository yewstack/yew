use nanoid::nanoid;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::constant::RAW_CARDS;
use crate::state::Card;

pub fn shuffle_cards() -> Vec<Card> {
    let mut raw_cards = RAW_CARDS;

    raw_cards.shuffle(&mut thread_rng());

    raw_cards
        .iter()
        .map(|&p| Card {
            id: nanoid!(),
            flipped: false,
            name: p,
        })
        .collect()
}
