use web_sys::MouseEvent;
use yew::prelude::*;
use yew::{function_component, html, Html, Properties};

use crate::constant::CardName;
use crate::state::{Card, RawCard};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub card: Card,
    pub on_flip: Callback<RawCard>,
}

#[function_component]
pub fn ChessboardCard(props: &Props) -> Html {
    let Props { card, on_flip } = props.clone();
    let Card { flipped, name, id } = card;

    let get_link_by_cardname = {
        match name {
            CardName::EightBall => "public/8-ball.png",
            CardName::Kronos => "public/kronos.png",
            CardName::BakedPotato => "public/baked-potato.png",
            CardName::Dinosaur => "public/dinosaur.png",
            CardName::Rocket => "public/rocket.png",
            CardName::SkinnyUnicorn => "public/skinny-unicorn.png",
            CardName::ThatGuy => "public/that-guy.png",
            CardName::Zeppelin => "public/zeppelin.png",
        }
        .to_string()
    };

    let onclick = move |e: MouseEvent| {
        e.stop_propagation();
        (!flipped).then(|| {
            on_flip.emit(RawCard {
                id: id.clone(),
                name,
            })
        });
    };

    html! {
      <div class="chess-board-card-container">
          <div class={classes!("card", flipped.then_some("flipped"))} {onclick}>
              <img class="front" src={get_link_by_cardname} alt="card" />
              <img class="back" src="public/back.png" alt="card" />
          </div>
      </div>
    }
}
