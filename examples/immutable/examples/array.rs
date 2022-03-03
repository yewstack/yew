use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::immutable::*;
use yew::prelude::*;

struct MyComponent;

#[derive(Properties, PartialEq)]
struct MyComponentProps {
    values: IArray<IString>,
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
            <>
            <p>{"Hello to:"}</p>
            <ul>
            { for props.values.iter().map(|s| html!(<li>{s}</li>)) }
            </ul>
            </>
        }
    }
}

struct App {
    values: IArray<IString>,
}

enum AppMessage {
    AddName(String),
    Noop,
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            values: Default::default(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMessage::AddName(name) => {
                self.values = self
                    .values
                    .iter()
                    .chain(std::iter::once(IString::from(name)))
                    .collect();
                true
            }
            AppMessage::Noop => false,
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
                AppMessage::AddName(value)
            } else {
                AppMessage::Noop
            }
        });

        html! {
            <>
            <input {onkeyup} />
            <MyComponent values={&self.values} />
            </>
        }
    }
}

#[xtask_wasm::run_example]
fn run_app() {
    yew::start_app::<App>();
}
