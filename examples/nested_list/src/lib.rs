#![recursion_limit = "128"]

mod header;
mod item;
mod list;

use header::ListHeader;
use item::ListItem;
use list::{List, Msg as ListMsg};
use yew::prelude::*;

pub struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="main">
                <h1>{ "Nested List Demo" }</h1>
                <List>
                    <ListHeader text="Calling all Rusties!" on_hover=ListMsg::Hover />
                    <ListItem name="Rustin" on_hover=ListMsg::Hover />
                    <ListItem hide={true} name="Rustaroo" on_hover=ListMsg::Hover />
                    <ListItem name="Rustifer" on_hover=ListMsg::Hover>
                        <span>{"Hello!"}</span>
                    </ListItem>
                </List>
            </div>
        }
    }
}
