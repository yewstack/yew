extern crate yew_stdweb as yew;

fn main() {
    web_logger::init();
    yew::start_app::<multi_thread::Model>();
}
