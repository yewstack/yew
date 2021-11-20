#![recursion_limit="256"]

mod text_input;

mod app;
mod password;

use app::App;

fn main() {
    yew::start_app::<App>();
}