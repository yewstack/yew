use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{HtmlInputElement, InputEvent};
use yew::immutable::*;
use yew::prelude::*;

struct MyComponent;

#[derive(Properties, PartialEq)]
struct MyComponentProps {
    name: IString,
}

impl Component for MyComponent {
    type Message = ();
    type Properties = MyComponentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        html! {
            <p>{"Hello "}{&props.name}{"!"}</p>
        }
    }
}

struct App {
    name: IString,
}

enum AppMessage {
    UpdateName(String),
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            name: "World".into(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMessage::UpdateName(name) => {
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
            AppMessage::UpdateName(target.value())
        });

        html! {
            <>
            <input value={&self.name} {oninput} />
            <MyComponent name={&self.name} />
            </>
        }
    }
}

#[xtask_wasm::run_example]
fn run_app() {
    yew::start_app::<App>();
}
