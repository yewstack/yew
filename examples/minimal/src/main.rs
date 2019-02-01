fn main() {
    yew::initialize();
    yew::App::<minimal::Model>::new().mount_to_body();
    yew::run_loop();
}
