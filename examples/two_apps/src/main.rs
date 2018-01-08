extern crate stdweb;
#[macro_use]
extern crate yew;

use std::rc::Rc;
use std::cell::RefCell;
use stdweb::web::document;
use yew::html::*;

struct Context {
    senders: Vec<ScopeSender<Context, Msg>>,
}

struct Model {
    sender: ScopeSender<Context, Msg>,
    selector: &'static str,
    title: String,
}

enum Msg {
    SendToOpposite(String),
    SetTitle(String),
}

impl Component<Context> for Model {
    type Msg = Msg;

    fn create(context: &mut ScopeRef<Context, Msg>) -> Self {
        let sender = context.senders.pop().unwrap();
        Model {
            // TODO Need properties here...
            sender,
            selector: "",
            title: "Nothing".into(),
        }
    }

    fn update(&mut self, msg: Msg, _: &mut ScopeRef<Context, Msg>) {
        match msg {
            Msg::SendToOpposite(title) => {
                self.sender.send(Msg::SetTitle(title));
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

fn mount_app(selector: &'static str, app: Scope<Context, Msg>) {
    let element = document().query_selector(selector).unwrap();
    app.mount::<Model>(element);
}

fn main() {
    yew::initialize();

    let context = Context {
        senders: Vec::new(),
    };
    let context = Rc::new(RefCell::new(context));

    let mut first_app = Scope::reuse(context.clone());
    let to_first = first_app.sender();
    context.borrow_mut().senders.push(to_first);

    let mut second_app = Scope::reuse(context.clone());
    let to_second = second_app.sender();
    context.borrow_mut().senders.push(to_second);

    mount_app(".first-app", first_app);
    mount_app(".second-app", second_app);

    yew::run_loop();
}
