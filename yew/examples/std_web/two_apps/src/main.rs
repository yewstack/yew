use stdweb::web::IParentNode;
use two_apps_std_web::{Model, Msg};
use yew::html::Scope;
use yew::App;

fn mount_app(selector: &'static str, app: App<Model>) -> Scope<Model> {
    let document = yew::utils::document();
    let element = document.query_selector(selector).unwrap().unwrap();
    app.mount(element)
}

fn main() {
    yew::initialize();
    let first_app = App::new();
    let second_app = App::new();
    let to_first = mount_app(".first-app", first_app);
    let to_second = mount_app(".second-app", second_app);
    to_first.send_message(Msg::SetOpposite(to_second.clone()));
    to_second.send_message(Msg::SetOpposite(to_first.clone()));
    yew::run_loop();
}
