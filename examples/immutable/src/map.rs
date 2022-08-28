use implicit_clone::unsync::*;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct DisplayProps {
    values: IMap<u32, IString>,
}

#[function_component]
fn Display(props: &DisplayProps) -> Html {
    html! {
        <>
        <p>{"Hello to:"}</p>
        <ul>
        { for props.values.iter().map(|(i, s)| html!(<li>{i}{" => "}{s}</li>)) }
        </ul>
        </>
    }
}

pub struct MapExample {
    values: IMap<u32, IString>,
}

pub enum MapExampleMessage {
    AddName(String),
    Noop,
}

impl Component for MapExample {
    type Message = MapExampleMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            values: Default::default(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MapExampleMessage::AddName(name) => {
                self.values = self
                    .values
                    .iter()
                    .chain(std::iter::once((
                        self.values.len() as u32,
                        IString::from(name),
                    )))
                    .collect();
                true
            }
            MapExampleMessage::Noop => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let onkeyup = link.callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                let event: Event = e.dyn_into().unwrap_throw();
                let event_target = event.target().unwrap_throw();
                let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
                let value = target.value();
                target.set_value("");
                MapExampleMessage::AddName(value)
            } else {
                MapExampleMessage::Noop
            }
        });

        html! {
            <>
            <h2>{"Input"}</h2>
            <input {onkeyup} />
            <h2>{"Output"}</h2>
            <Display values={&self.values} />
            </>
        }
    }
}
