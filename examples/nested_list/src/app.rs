use std::iter;

use yew::prelude::*;

use super::header::ListHeader;
use super::item::ListItem;
use super::list::List;
use super::{Hovered, WeakComponentLink};

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
            .map(|letter| html_nested! { <ListItem key={format!("letter-{}", letter)} name={letter.to_string()} {on_hover} /> });

        html! {
            <div class="main" {onmouseover}>
                <h1>{ "Nested List Demo" }</h1>
                <List
                    {on_hover}
                    weak_link={list_link}
                    header={
                        vec![
                            html_nested! {
                                <ListHeader text="Calling all Rusties!" {on_hover} {list_link} key="header" />
                            }
                        ]
                    }
                >
                    {vec![
                        html_nested! { <ListItem key="rustin" name="Rustin" {on_hover} /> },
                        html_nested! { <ListItem key="rustaroo" hide=true name="Rustaroo" {on_hover} /> },
                        html_nested! {
                            <ListItem key="rustifer" name="Rustifer" {on_hover}>
                                <div class="sublist" onmouseover={onmouseoversublist}>{ "Sublist!" }</div>
                                <List
                                    {on_hover}
                                    weak_link={sub_list_link}
                                    header={
                                        vec![html_nested! {
                                            <ListHeader key="sub-rusties" text="Sub Rusties!" {on_hover} list_link={sub_list_link}/>
                                        }]
                                    }
                                >
                                    {
                                        iter::once(html_nested! { <ListItem key="hidden-sub" hide=true name="Hidden Sub" {on_hover} /> })
                                        .chain(letters)
                                        .collect::<Vec<_>>()
                                    }
                                </List>
                            </ListItem>
                        },
                    ]}


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
