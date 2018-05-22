//! This demo originally created by https://github.com/qthree
//! Source: https://github.com/qthree/yew_table100x100_test

#[macro_use]
extern crate yew;

use yew::prelude::*;

pub struct Model {
    selected: Option<(u32, u32)>
}

pub enum Msg {
    Select(u32, u32),
}

impl<CTX> Component<CTX> for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: (), _: &mut Env<CTX, Self>) -> Self {
        Model {
            selected: None
        }
    }

    // Some details omitted. Explore the examples to get more.
    fn update(&mut self, msg: Self::Message, _: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::Select(x, y) => {
                self.selected = Some((x, y));
            }
        }
        true
    }
}

fn square_class(this: (u32, u32), selected: Option<(u32, u32)>) -> &'static str {
    match selected {
        Some(xy) if xy == this => {"square_green"},
        _ => {"square_red"}
    }
}

fn view_square<CTX>(selected: Option<(u32, u32)>, row: u32, column: u32) -> Html<CTX, Model>
where
    CTX: 'static,
{
    html! {
        <td
            class=square_class((column, row), selected),
            onclick=|_| Msg::Select(column, row),
        >
        </td>
    }
}

fn view_row<CTX>(selected: Option<(u32, u32)>, row: u32) -> Html<CTX, Model>
where
    CTX: 'static,
{
    html! {
        <tr>
            {for (0..99).map(|column| {
                view_square(selected, row, column)
            })}
        </tr>
    }
}

impl<CTX> Renderable<CTX, Model> for Model
where
    CTX: 'static,
{
    fn view(&self) -> Html<CTX, Self> {
        html! {
            <table>
                {for (0..99).map(|row| {
                    view_row(self.selected, row)
                })}
            </table>
        }
    }
}
