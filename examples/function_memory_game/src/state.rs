use std::rc::Rc;

use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use yew::prelude::*;

use crate::constant::{CardName, Status, KEY_BEST_SCORE};
use crate::helper::shuffle_cards;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct RawCard {
    pub id: String,
    pub name: CardName,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Card {
    pub id: String,
    pub flipped: bool,
    pub name: CardName,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct State {
    pub unresolved_card_pairs: u8,
    pub best_score: u32,
    pub status: Status,
    pub cards: Vec<Card>,
    pub last_card: Option<RawCard>,
    pub rollback_cards: Option<[RawCard; 2]>,
}

impl PartialEq<RawCard> for &mut Card {
    fn eq(&self, other: &RawCard) -> bool {
        self.id == other.id && self.name == other.name
    }
}

pub enum Action {
    FlipCard(RawCard),
    RollbackCards([RawCard; 2]),
    TrySaveBestScore(u32),
    GameReset,
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::FlipCard(card) => {
                let status = if self.status == Status::Ready {
                    Status::Playing
                } else {
                    self.status
                };

                let mut cards = self.cards.clone();
                cards.iter_mut().filter(|c| c.eq(&card)).for_each(|c| {
                    c.flipped = !c.flipped;
                });

                let last_card = self.last_card.clone();

                match last_card {
                    None => State {
                        unresolved_card_pairs: self.unresolved_card_pairs,
                        best_score: self.best_score,
                        status,
                        cards: cards.clone(),
                        last_card: Some(card),
                        rollback_cards: None,
                    },
                    Some(last_card) => {
                        let mut unresolved_card_pairs = self.unresolved_card_pairs;
                        let mut status = self.status;
                        let mut rollback_cards = self.rollback_cards.clone();
                        if card.id.ne(&last_card.id) && card.name.eq(&last_card.name) {
                            unresolved_card_pairs = self.unresolved_card_pairs - 1;
                            status = if unresolved_card_pairs == 0 {
                                Status::Passed
                            } else {
                                self.status
                            };
                        } else {
                            rollback_cards = Some([last_card, card]);
                        }

                        State {
                            unresolved_card_pairs,
                            best_score: self.best_score,
                            status,
                            cards: cards.clone(),
                            last_card: None,
                            rollback_cards,
                        }
                    }
                }
                .into()
            }
            Action::RollbackCards(rollback_cards) => {
                let mut cards = self.cards.clone();

                cards
                    .iter_mut()
                    .filter(|c| {
                        rollback_cards.contains(
                            &(RawCard {
                                id: c.id.clone(),
                                name: c.name,
                            }),
                        )
                    })
                    .for_each(|c| {
                        c.flipped = !c.flipped;
                    });

                State {
                    unresolved_card_pairs: self.unresolved_card_pairs,
                    best_score: self.best_score,
                    status: self.status,
                    cards,
                    last_card: self.last_card.clone(),
                    rollback_cards: None,
                }
                .into()
            }
            Action::TrySaveBestScore(sec_past) => {
                (self.best_score > sec_past).then(|| LocalStorage::set(KEY_BEST_SCORE, sec_past));
                self
            }
            Action::GameReset => State::reset().into(),
        }
    }
}

impl State {
    pub fn reset() -> State {
        State {
            unresolved_card_pairs: 8,
            best_score: LocalStorage::get(KEY_BEST_SCORE).unwrap_or(9999),
            status: Status::Ready,
            cards: shuffle_cards(),
            last_card: None,
            rollback_cards: None,
        }
    }
}
