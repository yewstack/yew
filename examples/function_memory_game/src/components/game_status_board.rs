use crate::constant::Status;
use yew::prelude::*;
use yew::{function_component, html, Properties};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub status: Status,
    pub sec_past: u32,
    pub on_reset: Callback<()>,
}

#[function_component(GameStatusBoard)]
pub fn game_status_board(props: &Props) -> Html {
    let get_content = {
        let onclick = props.on_reset.reform(move |e: MouseEvent| {
            e.stop_propagation();
            e.prevent_default();
        });

        match props.status {
            Status::Ready => html! {
                <span >{"Ready"}</span>
            },
            Status::Playing => html! {
                <span >{"Playing"}</span>
            },
            Status::Passed => html! {
                <a onclick={onclick}>{"Play again"}</a>
            },
        }
    };

    html! {
      <div class="game-status-container">
        {get_content}
        <span class="sec-past">{ props.sec_past}{" s"}</span>
    </div>
    }
}
