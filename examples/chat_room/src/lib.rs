#![recursion_limit = "512"]
use web_sys::{HtmlInputElement, InputEvent};
extern crate strum;
extern crate wasm_bindgen;
extern crate web_sys;
#[macro_use]
extern crate serde_derive;
extern crate yew;
#[macro_use]
extern crate log;
#[macro_use]
extern crate stdweb;
use std::collections::HashSet;
use std::env;

use services::PubnubService;
use yew::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub text: String,
    pub from: String,
}

pub struct Model {
    alias: String,
    pending_text: String,
    messages: Vec<Message>,
    users: HashSet<String>,
    pubnub: PubnubService,
}

#[derive(Debug)]
pub enum Msg {
    SendChat,
    AddMessage(Message),
    Connect,
    EnterName(String),
    UserOffline(String),
    UserOnline(String),
    UpdatePendingText(String),
    NoOp,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Model {
            messages: vec![],
            alias: "".to_string(),
            users: HashSet::new(),
            pending_text: "".to_string(),
            pubnub: PubnubService::new(env!("PUB_KEY_0"), env!("SUB_KEY_0")),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddMessage(msg) => {
                self.messages.push(msg);
            }
            Msg::UserOnline(nick) => {
                info!("Adding user {:?}", nick);
                self.users.insert(nick);
            }
            Msg::UserOffline(nick) => {
                info!("Removing user {:?}", nick);
                self.users.remove(&nick);
            }
            Msg::SendChat => {
                info!("Called send chat!");
                self.pubnub.send_message("chat-room", &self.pending_text);
                self.pending_text = "".into();
            }
            Msg::Connect => {
                let on_message = ctx.link().callback(|msg| Msg::AddMessage(msg));
                let onoffline = ctx.link().callback(|user| Msg::UserOffline(user));
                let ononline = ctx.link().callback(|user| Msg::UserOnline(user));
                self.pubnub
                    .connect("chat-room", &self.alias, on_message, onoffline, ononline);
            }
            Msg::EnterName(n) => {
                self.alias = n;
            }
            Msg::UpdatePendingText(s) => {
                self.pending_text = s;
            }
            Msg::NoOp => {}
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_connect_click = ctx.link().callback(|_| Msg::Connect);
        let on_alias_input_change = ctx.link().callback(|e: InputEvent| {
            Msg::EnterName(e.target_unchecked_into::<HtmlInputElement>().value())
        });
        let on_input_pending_text = ctx.link().callback(|e: InputEvent| {
            Msg::UpdatePendingText(e.target_unchecked_into::<HtmlInputElement>().value())
        });

        let onkeypress = ctx.link().callback(|event: KeyboardEvent| {
            if event.key() == "Enter" {
                Msg::SendChat
            } else {
                Msg::NoOp
            }
        });
        let on_send_msg_click = ctx.link().callback(|_| Msg::SendChat);

        html! {
            <div class="container">
              <div class="chat-container">
              <div class="chat-messages">
                <h2>{ "Messages" }</h2>
                <ul>
                  { for self.messages.iter().enumerate().map(view_message) }
                </ul>
                <div class="chat-inputs">
                  <input
                    placeholder="Type your message.."
                    type="text"
                    class="pending-text"
                    value={self.pending_text.clone()}
                    oninput={on_input_pending_text}
                    onkeypress={onkeypress}
                  />
                  <button type="submit" onclick={on_send_msg_click}>{"Send"}</button>
                </div>
              </div>
              <div class="chat-users">
                <h2>{ "Users" }</h2>
                <ul>
                  { for self.users.iter().enumerate().map(view_user::<dyn AsMut<PubnubService> + 'static>) }
                </ul>
                <div class="username-input">
                  <input
                    placeholder="Enter username..."
                    type="text"
                    value={self.alias.clone()}
                    oninput={on_alias_input_change}
                  />
                  <button type="submit" onclick={on_connect_click}>{ "Join" }</button>
                </div>
              </div>
            </div>
          </div>
        }
    }
}

fn view_message((_idx, message): (usize, &Message)) -> Html {
    html! {
      <li>
        <div class="message sent">
          <span class="sender">{"["}{&message.from}{"]"}</span>
          <span class="text">{&message.text}</span>
        </div>
      </li>
    }
}

fn view_user<C: ?Sized>((_idx, user): (usize, &String)) -> Html {
    html! {
      <li>
        <span>{ user }</span>
      </li>
    }
}

pub mod services;
