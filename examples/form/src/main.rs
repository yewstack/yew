#![recursion_limit = "1024"]

mod form;
mod text_input;

fn main() {
    yew::initialize();
    yew::start_app::<form::Form>();
}
