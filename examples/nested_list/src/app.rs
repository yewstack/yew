use super::header::ListHeader;
use super::item::ListItem;
use super::list::List;
use super::{Hovered, WeakContextRef};
use yew::prelude::*;

pub enum Msg {
    Hover(Hovered),
}

pub struct App {
    hovered: Hovered,
    list_ref: WeakContextRef<List>,
    sub_list_ref: WeakContextRef<List>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            hovered: Hovered::None,
            list_ref: WeakContextRef::default(),
            sub_list_ref: WeakContextRef::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Hover(hovered) => {
                self.hovered = hovered;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_hover = &ctx.callback(Msg::Hover);
        let onmouseenter = &ctx.callback(|_| Msg::Hover(Hovered::None));
        let list_link = &self.list_ref;
        let sub_list_link = &self.sub_list_ref;

        // note the use of `html_nested!` instead of `html!`.
        let letters = ('A'..='C')
            .map(|letter| html_nested! { <ListItem name=letter.to_string() on_hover=on_hover /> });

        html! {
            <div class="main" onmouseenter=onmouseenter>
                <h1>{ "Nested List Demo" }</h1>
                <List on_hover=on_hover weak_link=list_link>
                    <ListHeader text="Calling all Rusties!" on_hover=on_hover list_link=list_link />
                    <ListItem name="Rustin" on_hover=on_hover />
                    <ListItem hide=true name="Rustaroo" on_hover=on_hover />
                    <ListItem name="Rustifer" on_hover=on_hover>
                        <div class="sublist">{ "Sublist!" }</div>
                        <List on_hover=on_hover weak_link=sub_list_link>
                            <ListHeader text="Sub Rusties!" on_hover=on_hover list_link=sub_list_link/>
                            <ListItem hide=true name="Hidden Sub" on_hover=on_hover />
                            { for letters }
                        </List>
                    </ListItem>
                </List>
                { self.view_last_hovered() }
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
