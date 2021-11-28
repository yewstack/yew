use super::header::ListHeader;
use super::item::ListItem;
use super::list::List;
use super::{Hovered, WeakComponentLink};
use yew::prelude::*;

pub enum Msg {
    Hover(Hovered),
}

pub struct App {
    hovered: Hovered,
    list_link: WeakComponentLink<List>,
    sub_list_link: WeakComponentLink<List>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            hovered: Hovered::None,
            list_link: WeakComponentLink::default(),
            sub_list_link: WeakComponentLink::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Hover(hovered) => {
                self.hovered = hovered;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_hover = &ctx.link().callback(Msg::Hover);
        let onmouseover = &ctx.link().callback(|_| Msg::Hover(Hovered::None));
        let onmouseoversublist = &ctx.link().callback(|e: MouseEvent| {
            e.stop_propagation();
            Msg::Hover(Hovered::List)
        });
        let list_link = &self.list_link;
        let sub_list_link = &self.sub_list_link;

        // note the use of `html_nested!` instead of `html!`.
        let letters = ('A'..='C')
            .map(|letter| html_nested! { <ListItem name={letter.to_string()} {on_hover} /> });

        html! {
            <div class="main" {onmouseover}>
                <h1>{ "Nested List Demo" }</h1>
                <List {on_hover} weak_link={list_link}>
                    <ListHeader text="Calling all Rusties!" {on_hover} {list_link} />
                    <ListItem name="Rustin" {on_hover} />
                    <ListItem hide=true name="Rustaroo" {on_hover} />
                    <ListItem name="Rustifer" {on_hover}>
                        <div class="sublist" onmouseover={onmouseoversublist}>{ "Sublist!" }</div>
                        <List {on_hover} weak_link={sub_list_link}>
                            <ListHeader text="Sub Rusties!" {on_hover} list_link={sub_list_link}/>
                            <ListItem hide=true name="Hidden Sub" {on_hover} />
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
