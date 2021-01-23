//! This module contains the implementation of a service
//! to show alerts and confirm dialogs in a browser.
//!
//! If you call these methods repeatably browsers tend to disable these options to give users
//! a better experience.

use yew::utils;

/// A dialog service.
#[derive(Default, Debug)]
pub struct DialogService {}

impl DialogService {
    /// Calls [alert](https://developer.mozilla.org/en-US/docs/Web/API/Window/alert)
    /// function.
    pub fn alert(message: &str) {
        utils::window().alert_with_message(message).unwrap()
    }

    /// Calls [confirm](https://developer.mozilla.org/en-US/docs/Web/API/Window/confirm)
    /// function.
    pub fn confirm(message: &str) -> bool {
        utils::window().confirm_with_message(message).unwrap()
    }

    /// Prompts the user to input a message. In most browsers this will open an alert box with
    /// an input field where the user can input a message.
    #[doc = "A default value can be supplied which will be returned if the user doesn't input anything."]
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
    #[doc = "This function will return `None` if the value of `default` is `None` and the user \
        cancels the operation. (normally a 'cancel' button will be displayed to the user, \
        clicking which cancels the operation)."]
    pub fn prompt(message: &str, default: Option<&str>) -> Option<String> {
        if let Some(default) = default {
            utils::window()
                .prompt_with_message_and_default(message, default)
                .expect("Couldn't read input.")
        } else {
            utils::window()
                .prompt_with_message(message)
                .expect("Couldn't read input.")
        }
    }
}
