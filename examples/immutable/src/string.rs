use implicit_clone::unsync::*;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlInputElement, InputEvent};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct DisplayProps {
    name: IString,
}

#[function_component]
fn Display(props: &DisplayProps) -> Html {
    html! {
        <p>{"Hello "}{&props.name}{"!"}</p>
    }
}

pub struct StringExample {
    name: IString,
}

pub enum StringExampleMessage {
    UpdateName(String),
}

impl Component for StringExample {
    type Message = StringExampleMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            name: "World".into(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            StringExampleMessage::UpdateName(name) => {
                self.name = name.into();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let oninput = link.callback(|e: InputEvent| {
            let event: Event = e.dyn_into().unwrap_throw();
            let event_target = event.target().unwrap_throw();
            let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
            StringExampleMessage::UpdateName(target.value())
        });

        html! {
            <>
            <h2>{"Input"}</h2>
            <input value={&self.name} {oninput} />
            <h2>{"Output"}</h2>
            <Display name={&self.name} />
            </>
        }
    }
}
