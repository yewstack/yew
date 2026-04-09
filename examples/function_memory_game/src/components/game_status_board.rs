use yew::prelude::*;
use yew::{Properties, function_component, html};

use crate::constant::Status;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub status: Status,
    pub sec_past: u32,
    pub on_reset: Callback<()>,
}

#[function_component]
pub fn GameStatusBoard(props: &Props) -> Html {
    html! {
      <div class="game-status-container">
        match props.status {
            Status::Ready => <span>{"Ready"}</span>,
            Status::Playing => <span>{"Playing"}</span>,
            Status::Passed => {
                let onclick = props.on_reset.reform(move |e: MouseEvent| {
                    e.stop_propagation();
                    e.prevent_default();
                });
                <button class="play-again-btn" {onclick}>{"Play again"}</button>
            }
        }
        <span class="sec-past">{ props.sec_past}{" s"}</span>
    </div>
    }
}
