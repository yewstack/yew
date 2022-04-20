extern crate zxcvbn;

use yew::prelude::*;
use zxcvbn::zxcvbn;

use crate::password::generate_password;
use crate::text_input::TextInput;

pub enum Msg {
    SetPassword(String),
    RegeneratePassword,
}

#[derive(Debug, Default)]
pub struct App {
    password: String,
}

impl App {
    fn get_estimate(&self) -> Option<u8> {
        zxcvbn(&self.password, &[])
            .ok()
            .map(|estimate| estimate.score())
    }

    fn redout_top_row_text(&self) -> String {
        if self.password.is_empty() {
            return "Provide a password".to_string();
        }
        let estimate_text = match self.get_estimate().unwrap_or(0) {
            0 => "That's a password?",
            1 => "You can do a lot better.",
            2 => "Meh",
            3 => "Good",
            _ => "Great!",
        };
        format!("Complexity = {}", estimate_text)
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetPassword(next_password) => self.password = next_password,
            Msg::RegeneratePassword => self.password = generate_password(),
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change = ctx.link().callback(Msg::SetPassword);
        let onclick = ctx.link().callback(|_| Msg::RegeneratePassword);
        html! {
            <main>
                <div class="entry">
                    <div>
                        {"Enter a password below:"}
                        <div class="footnote">
                            {"(Will show in clear text)"}
                        </div>
                    </div>
                    <div>
                        <TextInput {on_change} value={self.password.clone()} />
                    </div>
                </div>
                <div class="readout">
                    <div>
                        {self.redout_top_row_text()}
                    </div>
                    <button {onclick}>
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
