//! This demo originally created by https://github.com/qthree
//! Source: https://github.com/qthree/yew_table100x100_test

use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Model {
    link: ComponentLink<Self>,
    selected: Option<(u32, u32)>,
}

pub enum Msg {
    Select(u32, u32),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: (), link: ComponentLink<Self>) -> Self {
        Model {
            link,
            selected: None,
        }
    }

    // Some details omitted. Explore the examples to get more.
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Select(x, y) => {
                self.selected = Some((x, y));
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <table>
                { (0..99).map(|row| self.view_row(row)).collect::<Html>() }
            </table>
        }
    }
}

impl Model {
    fn view_square(&self, row: u32, column: u32) -> Html {
        html! {
            <td class=square_class((column, row), self.selected)
                onclick=self.link.callback(move |_| Msg::Select(column, row))>
            </td>
        }
    }

    fn view_row(&self, row: u32) -> Html {
        html! {
            <tr>
                {for (0..99).map(|column| {
                    self.view_square(row, column)
                })}
            </tr>
        }
    }
}

fn square_class(this: (u32, u32), selected: Option<(u32, u32)>) -> &'static str {
    match selected {
        Some(xy) if xy == this => "square_green",
        _ => "square_red",
    }
}
