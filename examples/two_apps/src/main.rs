/// This example demonstrates low-level usage of scopes.

extern crate stdweb;
#[macro_use]
extern crate yew;

use std::rc::Rc;
use std::cell::RefCell;
use stdweb::web::document;
// Use `html` module directly. No use `App`.
use yew::html::*;

struct Context {
    senders: Vec<ScopeSender<Context, Model>>,
}

struct Model {
    sender: ScopeSender<Context, Model>,
    selector: &'static str,
    title: String,
}

enum Msg {
    SendToOpposite(String),
    SetTitle(String),
}

impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        let sender = context.senders.pop().unwrap();
        Model {
            // TODO Use properties to set sender...
            sender,
            selector: "",
            title: "Nothing".into(),
        }
    }

    fn update(&mut self, msg: Msg, _: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::SendToOpposite(title) => {
                self.sender.send(ComponentUpdate::Message(Msg::SetTitle(title)));
            }
            Msg::SetTitle(title) => {
                self.title = title;
            }
        }
        true
    }
}


impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
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

fn mount_app(selector: &'static str, app: Scope<Context, Model>) {
    let element = document().query_selector(selector).unwrap();
    app.mount(element);
}

fn main() {
    yew::initialize();

    let context = Context {
        senders: Vec::new(),
    };

    // Example how to reuse context in two scopes
    let context = Rc::new(RefCell::new(context));

    let mut first_app = Scope::reuse(context.clone());
    let to_first = first_app.get_env().sender();
    context.borrow_mut().senders.push(to_first);

    let mut second_app = Scope::reuse(context.clone());
    let to_second = second_app.get_env().sender();
    context.borrow_mut().senders.push(to_second);

    mount_app(".first-app", first_app);
    mount_app(".second-app", second_app);

    yew::run_loop();
}
