use web_sys::{FormData, HtmlFormElement};
use yew::prelude::*;

pub enum Msg {
    Submit(SubmitEvent),
}

pub struct App {
    names: Vec<String>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            names: Vec::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Submit(event) => {
                event.prevent_default();
                let form: HtmlFormElement = event.target_unchecked_into();
                let form_data = FormData::new_with_form(&form).expect("form data");
                self.names.push(format!(
                    "{} {}",
                    form_data.get("first").as_string().unwrap(),
                    form_data.get("last").as_string().unwrap()
                ));
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <form onsubmit={ctx.link().callback(Msg::Submit)}>
                    <label>{"Sign up"}</label>
                    <input name="first" placeholder="First name"/>
                    <input name="last" placeholder="Last name"/>
                    <input type="submit"/>
                </form>
                <ul>
                    { for self.names.iter().map( |name| html! {
                        <li>{ name }</li>
                    })}
                </ul>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
