mod input;

use input::InputComponent;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub enum Msg {
    HoverIndex(usize),
    Submit,
}

pub struct App {
    refs: Vec<NodeRef>,
    focus_index: usize,
    email_error: String,
    password_error: String,
}
impl App {
    fn apply_focus(&self) {
        if let Some(input) = self.refs[self.focus_index].cast::<HtmlInputElement>() {
            input.focus().unwrap();
        }
    }
}
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            focus_index: 0,
            refs: vec![NodeRef::default(), NodeRef::default()],
            email_error: "".to_string(),
            password_error: "".to_string(),
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.apply_focus();
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HoverIndex(index) => {
                self.focus_index = index;
                self.apply_focus();
                false
            }
            Msg::Submit => {
                let email = &self.refs[0];
                let password = &self.refs[1];
                let email_value = email.cast::<HtmlInputElement>().unwrap().value();
                let password_value = password.cast::<HtmlInputElement>().unwrap().value();

                self.email_error.clear();
                self.password_error.clear();

                if !(email_value.contains('@') && email_value.contains('.')) {
                    self.email_error.push_str("Invalid email.")
                }
                if password_value.len() < 8 {
                    self.password_error
                        .push_str("Password must be at least 8 characters long.")
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="main">
                <div id="left-pane">
                    <div>
                        <h1>{"Create your account"}</h1>
                        <div class="input-container">
                            <label>{ "Email" }</label>
                            <input
                                type="text"
                                ref={&self.refs[0]}
                                class="input-element"
                                onmouseover={ctx.link().callback(|_| Msg::HoverIndex(0))}
                                placeholder="abcd@xyz.com"
                            />
                            <div class="error">{self.email_error.clone()}</div>
                        </div>
                        <div class="input-container">
                            <label>{ "Password" }</label>
                            <InputComponent
                                input_ref={&self.refs[1]}
                                on_hover={ctx.link().callback(|_| Msg::HoverIndex(1))}
                                placeholder="password"
                            />
                            <div class="error">{self.password_error.clone()}</div>
                        </div>
                        <button onclick={ctx.link().callback(|_| Msg::Submit)}>{"Create"}</button>
                    </div>
                </div>
                <div id="right-pane">
                    <div>
                        <div id="graphic"></div>
                        <h1>{ "Node Refs Example" }</h1>
                        <p>{ "Refs can be used to access and manipulate DOM elements directly" }</p>
                        <ul>
                            <li>{ "First input will focus on mount" }</li>
                            <li>{ "Each input will focus on hover" }</li>
                        </ul>
                    </div>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
