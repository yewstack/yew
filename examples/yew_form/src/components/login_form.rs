use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, HtmlInputElement, Window};
use yew::prelude::*;

use crate::api::auth::login_user;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct LoginUserSchema {
    email: String,
    password: String,
}

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let error_handle = use_state(String::default);
    let error = (*error_handle).clone();

    let input_email_ref = use_node_ref();
    let input_email_handle = use_state(String::default);
    let input_email = (*input_email_handle).clone();

    let input_password_ref = use_node_ref();
    let input_password_handle = use_state(String::default);
    let input_password = (*input_password_handle).clone();

    let on_email_change = {
        let input_email_ref = input_email_ref.clone();

        Callback::from(move |_| {
            let input = input_email_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_email_handle.set(value);
            }
        })
    };

    let on_password_change = {
        let input_password_ref = input_password_ref.clone();

        Callback::from(move |_| {
            let input = input_password_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_password_handle.set(value);
            }
        })
    };

    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();

        let email_ref = input_password.clone();
        let password_ref = input_password.clone();
        let error_handle = error_handle.clone();
        console::log_1(&format!("Email: {}, Password: {}", input_email, input_password).into());

        spawn_local(async move {
            let email_val = email_ref.clone();
            let password_val = password_ref.clone();
            let error_handle = error_handle.clone();
            let response = login_user(email_val, password_val).await;
            match response {
                Ok(_) => {
                    console::log_1(&"success".into());
                    let window: Window = web_sys::window().expect("window not available");
                    let location = window.location();
                    let _ = location.set_href("/error");
                }
                Err(err) => {
                    error_handle.set(err);
                }
            }
        });
    });

    html! {
        <div>
            <form
             onsubmit={onsubmit}
            >
              if !error.is_empty() {
                <div class="error">{error}</div>
              }
              <h1>{"Login"}</h1>
              <label for="username">{"Username"}</label>
              <div>
              <input
                  class="form-control form-control-lg"
                  type="text"
                  id="username"
                  name="username"
                  placeholder="Email"
                  ref={input_email_ref}
                  oninput={on_email_change}
              />
              </div>
              <label for="password">{"Password"}</label>
              <input
                  class="form-control form-control-lg"
                  type="password"
                  id="password"
                  name="password"
                  placeholder="Password"
                  ref={input_password_ref}
                  oninput={on_password_change}
              />
              <div class="forget-password-container">
                  <a href="#">{"Forgot Password?"}</a>
              </div>
              <input type="submit" value="Login" />
              <div class="bottom-container">
                <p>
                  {"Don't have an account?"}
                  <a href="#">{"Sign up"}</a>
                </p>
              </div>
            </form>
        </div>
    }
}
