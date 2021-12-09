use web_sys::MouseEvent;
use yew::prelude::*;
use yew::{function_component, html, Properties};

use crate::constant::CardName;
use crate::state::{Card, RawCard};

#[derive(Properties, Clone)]
pub struct Props {
    pub card: Card,
    pub on_flip: Callback<RawCard>,
}

impl PartialEq for Props {
    fn eq(&self, other: &Props) -> bool {
        self.card.eq(&other.card)
    }
}

#[function_component(ChessboardCard)]
pub fn chessboard_card(props: &Props) -> Html {
    let Card { flipped, name, id } = props.card.clone();

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

    let onclick = {
        let on_flip = props.on_flip.clone();

        move |e: MouseEvent| {
            e.stop_propagation();

            (!flipped).then(|| {
                on_flip.emit(RawCard {
                    id: id.clone(),
                    name,
                })
            });
        }
    };

    html! {
      <div class="chess-board-card-container">
          <div class={classes!("card", flipped.then(|| "flipped"))} {onclick}>
              <img class="front" src={get_link_by_cardname} />
              <img class="back" src="public/back.png" />
          </div>
      </div>
    }
}
