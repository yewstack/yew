extern crate zxcvbn;

use yew::prelude::*;
use zxcvbn::zxcvbn;

use crate::password::generate_password;
use crate::text_input::TextInput;

pub enum Msg {
    SetPassword(String),
    RegeneratePassword
}

#[derive(Debug)]
pub struct Model {
    link: ComponentLink<Self>,
    password: String,
}

pub type App = Model;

impl App {
    fn get_estimate(&self) -> Option<u8> {
        zxcvbn(self.password.as_ref(), &[]).ok().map(|estimate| estimate.score())
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            password: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetPassword(next_password) => {
                self.password = next_password
            },
            Msg::RegeneratePassword => {
                self.password = generate_password()
            }
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html!{
            <main>
                <div class="entry">
                    <div>
                        {"Enter a password below:"}
                        <div class="footnote">
                            {"(Will show in clear text)"}
                        </div>
                    </div>
                    <div>
                        <TextInput
                            onchange=self.link.callback(Msg::SetPassword)
                            value={self.password.clone()}
                        />
                    </div>
                </div>
                <div class="readout">
                    <div>
                        {"Complexity = "}
                        {match self.get_estimate().unwrap_or(0) {
                            0 => "That's a password?",
                            1 => "You can do a lot better.",
                            2 => "Meh",
                            3 => "Good",
                            _ => "Great!",
                        }}
                    </div>
                    <button onclick=self.link.callback(|_| Msg::RegeneratePassword)>
                        {"Generate new password *"}
                    </button>
                    <div class="footnote">
                        {"* Note: generated passwords are not actually cryptographically secure"}
                    </div>
                </div>
            </main>
        }
    }
}