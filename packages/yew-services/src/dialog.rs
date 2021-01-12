//! This module contains the implementation of a service
//! to show alerts and confirm dialogs in a browser.
//!
//! If you call these methods repeatably browsers tend to disable these options to give users
//! a better experience.

use cfg_if::cfg_if;
use cfg_match::cfg_match;
cfg_if! {
    if #[cfg(feature = "std_web")] {
        use stdweb::Value;
        #[allow(unused_imports)]
        use stdweb::{_js_impl, js};
    } else if #[cfg(feature = "web_sys")] {
        use yew::utils;
    }
}

/// A dialog service.
#[derive(Default, Debug)]
pub struct DialogService {}

impl DialogService {
    /// Calls [alert](https://developer.mozilla.org/en-US/docs/Web/API/Window/alert)
    /// function.
    pub fn alert(message: &str) {
        cfg_match! {
            feature = "std_web" => js! { @(no_return) alert(@{message}); },
            feature = "web_sys" => utils::window().alert_with_message(message).unwrap(),
        };
    }

    /// Calls [confirm](https://developer.mozilla.org/en-US/docs/Web/API/Window/confirm)
    /// function.
    pub fn confirm(message: &str) -> bool {
        cfg_match! {
            feature = "std_web" => ({
                let value: Value = js! { return confirm(@{message}); };
                match value {
                    Value::Bool(result) => result,
                    _ => false,
                }
            }),
            feature = "web_sys" => utils::window().confirm_with_message(message).unwrap(),
        }
    }

    /// Prompts the user to input a message. In most browsers this will open an alert box with
    /// an input field where the user can input a message.
    #[cfg_attr(
        feature = "web_sys",
        doc = "A default value can be supplied which will be returned if the user doesn't input anything."
    )]
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/prompt)
    ///
    /// This method will `panic!` if there is an error in the process of trying to carry out this
    /// operation.
    ///
    /// Note that this function is blocking; no other code can be run on the thread while
    /// the user inputs their message which means that the page will appear to have 'frozen'
    /// while the user types in their message.
    ///
    #[cfg_attr(
        feature = "web_sys",
        doc = "This function will return `None` if the value of `default` is `None` and the user \
        cancels the operation. (normally a 'cancel' button will be displayed to the user, \
        clicking which cancels the operation)."
    )]
    #[cfg_attr(
        feature = "std_web",
        doc = "This function will return `None` if the user cancels the operation (normally a \
        'cancel' button will be displayed to the user, clicking which cancels the operation)."
    )]
    pub fn prompt(
        message: &str,
        #[cfg(feature = "web_sys")] default: Option<&str>,
    ) -> Option<String> {
        cfg_if! {
            if #[cfg(feature="web_sys")] {
                if let Some(default) = default {
                    utils::window()
                           .prompt_with_message_and_default(message, default)
                           .expect("Couldn't read input.")
                }
                else {
                    utils::window()
                           .prompt_with_message(message)
                           .expect("Couldn't read input.")
                }
            } else if #[cfg(feature="std_web")] {
                let value: Value = js! { return prompt(@{message}); };
                match value {
                    Value::String(result) => Some(result),
                    _ => None,
                }
            }
        }
    }
}
