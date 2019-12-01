use super::header::ListHeader;
use super::item::ListItem;
use super::list::List;
use super::Hovered;
use yew::prelude::*;

pub struct App {
    link: ComponentLink<Self>,
    hovered: Hovered,
}

pub enum Msg {
    Hover(Hovered),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            hovered: Hovered::None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Hover(hovered) => self.hovered = hovered,
        }
        true
    }

    fn view(&self) -> Html {
        let on_hover = &self.link.callback(Msg::Hover);
        let onmouseenter = &self.link.callback(|_| Msg::Hover(Hovered::None));
        html! {
            <div class="main" onmouseenter=onmouseenter>
                <h1>{ "Nested List Demo" }</h1>
                <List on_hover=on_hover>
                    <ListHeader text="Calling all Rusties!" on_hover=on_hover />
                    <ListItem name="Rustin" on_hover=on_hover />
                    <ListItem hide={true} name="Rustaroo" on_hover=on_hover />
                    <ListItem name="Rustifer" on_hover=on_hover>
                        <div class="sublist">{"Sublist!"}</div>
                        {
                            html! {
                                <List on_hover=on_hover>
                                    <ListHeader text="Sub Rusties!" on_hover=on_hover />
                                    <ListItem name="Sub Rustin" on_hover=on_hover />
                                    <ListItem hide={true} name="Sub Rustaroo" on_hover=on_hover />
                                    <ListItem name="Sub Rustifer" on_hover=on_hover />
                                </List>
                            }
                        }
                    </ListItem>
                </List>
                {self.view_last_hovered()}
            </div>
        }
    }
}

impl App {
    fn view_last_hovered(&self) -> Html {
        html! {
            <div class="last-hovered">
                { "Last hovered:"}
                <span class="last-hovered-text">
                    { &self.hovered }
                </span>
            </div>
        }
    }
}
