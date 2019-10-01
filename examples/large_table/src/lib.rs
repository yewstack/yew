//! This demo originally created by https://github.com/qthree
//! Source: https://github.com/qthree/yew_table100x100_test

use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Model {
    selected: Option<(u32, u32)>,
}

pub enum Msg {
    Select(u32, u32),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: (), _: ComponentLink<Self>) -> Self {
        Model { selected: None }
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

    fn view(&self) -> Html<Self> {
        html! {
            <table>
                { (0..99).map(|row| view_row(self.selected, row)).collect::<Html<Self>>() }
            </table>
        }
    }
}

fn square_class(this: (u32, u32), selected: Option<(u32, u32)>) -> &'static str {
    match selected {
        Some(xy) if xy == this => "square_green",
        _ => "square_red",
    }
}

fn view_square(selected: Option<(u32, u32)>, row: u32, column: u32) -> Html<Model> {
    html! {
        <td class=square_class((column, row), selected)
            onclick=|_| Msg::Select(column, row)>
        </td>
    }
}

fn view_row(selected: Option<(u32, u32)>, row: u32) -> Html<Model> {
    html! {
        <tr>
            {for (0..99).map(|column| {
                view_square(selected, row, column)
            })}
        </tr>
    }
}
