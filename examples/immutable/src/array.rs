use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::immutable::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct DisplayProps {
    values: IArray<IString>,
}

#[function_component]
fn Display(props: &DisplayProps) -> Html {
    html! {
        <>
        <p>{"Hello to:"}</p>
        <ul>
        { for props.values.iter().map(|s| html!(<li>{s}</li>)) }
        </ul>
        </>
    }
}

pub struct ArrayExample {
    values: IArray<IString>,
}

pub enum ArrayExampleMessage {
    AddName(String),
    Noop,
}

impl Component for ArrayExample {
    type Message = ArrayExampleMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            values: Default::default(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ArrayExampleMessage::AddName(name) => {
                self.values = self
                    .values
                    .iter()
                    .chain(std::iter::once(IString::from(name)))
                    .collect();
                true
            }
            ArrayExampleMessage::Noop => false,
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
                ArrayExampleMessage::AddName(value)
            } else {
                ArrayExampleMessage::Noop
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
