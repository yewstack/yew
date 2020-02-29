fn main() {
    web_logger::init();
    yew::start_app::<multi_thread_std_web::Model>();
}
