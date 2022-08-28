use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

pub const KEY_BEST_SCORE: &str = "memory.game.best.score";

#[derive(Clone, Copy, Debug, EnumIter, Display, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardName {
    EightBall,
    Kronos,
    BakedPotato,
    Dinosaur,
    Rocket,
    SkinnyUnicorn,
    ThatGuy,
    Zeppelin,
}

#[derive(Clone, Copy, Debug, EnumIter, Display, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    Ready,
    Playing,
    Passed,
}

pub const RAW_CARDS: [CardName; 16] = [
    CardName::EightBall,
    CardName::Kronos,
    CardName::BakedPotato,
    CardName::Dinosaur,
    CardName::Rocket,
    CardName::SkinnyUnicorn,
    CardName::ThatGuy,
    CardName::Zeppelin,
    CardName::EightBall,
    CardName::Kronos,
    CardName::BakedPotato,
    CardName::Dinosaur,
    CardName::Rocket,
    CardName::SkinnyUnicorn,
    CardName::ThatGuy,
    CardName::Zeppelin,
];
