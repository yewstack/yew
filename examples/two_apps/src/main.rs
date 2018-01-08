extern crate stdweb;
#[macro_use]
extern crate yew;

use stdweb::web::document;
use yew::html::*;

type Context = ();

struct Model {
    sender: Option<ScopeSender<Context, Msg>>,
    selector: &'static str,
    title: String,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            sender: None,
            selector: "",
            title: "".into(),
        }
    }
}

enum Msg {
    SendToOpposite(String),
    SetTitle(String),
}

impl Component<()> for Model {
    type Msg = Msg;

    fn update(&mut self, msg: Msg, _: &mut ScopeRef<Context, Msg>) {
        match msg {
            Msg::SendToOpposite(title) => {
                self.sender.as_mut().unwrap().send(Msg::SetTitle(title));
            }
            Msg::SetTitle(title) => {
                self.title = title;
            }
        }
    }

    fn view(&self) -> Html<Context, Msg> {
        html! {
            <div>
                <h3>{ format!("{} received <{}>", self.selector, self.title) }</h3>
                <button onclick=|_| Msg::SendToOpposite("One".into()),>{ "One" }</button>
                <button onclick=|_| Msg::SendToOpposite("Two".into()),>{ "Two" }</button>
                <button onclick=|_| Msg::SendToOpposite("Three".into()),>{ "Three" }</button>
            </div>
        }
    }
}

fn mount_app(selector: &'static str, app: &mut Scope<Context, Msg>, sender: ScopeSender<Context, Msg>) {
    let model = Model {
        sender: Some(sender),
        selector,
        title: "Not set".into(),
    };
    let element = document().query_selector(selector).unwrap();
    app.mount_to(element, model);
}

fn main() {
    yew::initialize();

    let mut first_app = Scope::new(());
    let to_first = first_app.sender();

    let mut second_app = Scope::new(());
    let to_second = second_app.sender();

    mount_app(".first-app", &mut first_app, to_second);
    mount_app(".second-app", &mut second_app, to_first);

    yew::run_loop();
}
