#[macro_use]
extern crate yew;

use yew::html::*;

struct Context {
    sender: AppSender<Msg>,
}

struct Model {
    selector: &'static str,
    title: String,
}

enum Msg {
    SendToOpposite(String),
    SetTitle(String),
}

fn update(context: &mut Context, model: &mut Model, msg: Msg) {
    match msg {
        Msg::SendToOpposite(title) => {
            context.sender.send(Msg::SetTitle(title));
        }
        Msg::SetTitle(title) => {
            model.title = title;
        }
    }
}

fn view(model: &Model) -> Html<Msg> {
    html! {
        <div>
            <h3>{ format!("{} received <{}>", model.selector, model.title) }</h3>
            <button onclick=|_| Msg::SendToOpposite("One".into()),>{ "One" }</button>
            <button onclick=|_| Msg::SendToOpposite("Two".into()),>{ "Two" }</button>
            <button onclick=|_| Msg::SendToOpposite("Three".into()),>{ "Three" }</button>
        </div>
    }
}

fn mount_app(selector: &'static str, app: &mut App<Msg>, sender: AppSender<Msg>) {
    let context = Context {
        sender,
    };
    let model = Model {
        selector,
        title: "Not set".into(),
    };
    app.mount_to(selector, context, model, update, view);
}

fn main() {
    yew::initialize();

    let mut first_app = App::new();
    let to_first = first_app.sender();

    let mut second_app = App::new();
    let to_second = second_app.sender();

    mount_app(".first-app", &mut first_app, to_second);
    mount_app(".second-app", &mut second_app, to_first);

    yew::run_loop();
}
