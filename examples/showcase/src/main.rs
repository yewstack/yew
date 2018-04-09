#[macro_use]
extern crate yew;
extern crate counter;

use yew::prelude::*;
use yew::services::console::ConsoleService;
use counter::Model as Counter;

struct Context {
    console: ConsoleService,
}

impl AsRef<ConsoleService> for Context {
    fn as_ref(&self) -> &ConsoleService {
        &self.console
    }
}

enum Scene {
    NotSelected,
    Counter,
}

enum Msg {
    SwitchTo(Scene),
}

impl Component<Context> for Scene {
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        Scene::NotSelected
    }

    fn update(&mut self, msg: Self::Msg, _: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::SwitchTo(scene) => {
                *self = scene;
                true
            }
        }
    }
}

impl Renderable<Context, Scene> for Scene {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <p>{ "Showcase" }</p>
            { self.view_scene() }
            <button onclick=|_| Msg::SwitchTo(Scene::NotSelected),>{ "Back" }</button>
            <button onclick=|_| Msg::SwitchTo(Scene::Counter),>{ "Counter" }</button>
        }
    }
}

impl Scene {
    fn view_scene(&self) -> Html<Context, Self> {
        match *self {
            Scene::NotSelected => {
                html! {
                    <p>{ "Select the scene, please." }</p>
                }
            }
            Scene::Counter => {
                html! {
                    <Counter: />
                }
            }
        }
    }
}

fn main() {
    yew::initialize();
    let context = Context {
        console: ConsoleService,
    };
    let app: App<_, Scene> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}

