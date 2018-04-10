#[macro_use]
extern crate yew;
extern crate counter;
extern crate crm;
extern crate custom_components;
extern crate dashboard;
extern crate fragments;

use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::dialog::DialogService;
use yew::services::storage::{StorageService, Area};
use yew::services::fetch::FetchService;
use yew::services::websocket::WebSocketService;
use counter::Model as Counter;
use crm::Model as Crm;
use custom_components::Model as CustomComponents;
use dashboard::Model as Dashboard;
use fragments::Model as Fragments;

struct Context {
    console: ConsoleService,
    storage: StorageService,
    dialog: DialogService,
    web: FetchService,
    ws: WebSocketService,
}

impl AsMut<ConsoleService> for Context {
    fn as_mut(&mut self) -> &mut ConsoleService {
        &mut self.console
    }
}

impl AsMut<StorageService> for Context {
    fn as_mut(&mut self) -> &mut StorageService {
        &mut self.storage
    }
}

impl AsMut<DialogService> for Context {
    fn as_mut(&mut self) -> &mut DialogService {
        &mut self.dialog
    }
}

impl AsMut<FetchService> for Context {
    fn as_mut(&mut self) -> &mut FetchService {
        &mut self.web
    }
}

impl AsMut<WebSocketService> for Context {
    fn as_mut(&mut self) -> &mut WebSocketService {
        &mut self.ws
    }
}

impl custom_components::Printer for Context {
    fn print(&mut self, data: &str) {
        self.console.log(data);
    }
}

enum Scene {
    NotSelected,
    Counter,
    Crm,
    CustomComponents,
    Dashboard,
    Fragments,
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
            <button onclick=|_| Msg::SwitchTo(Scene::NotSelected),>{ "Home" }</button>
            <button onclick=|_| Msg::SwitchTo(Scene::Counter),>{ "Counter" }</button>
            <button onclick=|_| Msg::SwitchTo(Scene::Crm),>{ "Crm" }</button>
            <button onclick=|_| Msg::SwitchTo(Scene::CustomComponents),>{ "CustomComponents" }</button>
            <button onclick=|_| Msg::SwitchTo(Scene::Dashboard),>{ "Dashboard" }</button>
            <button onclick=|_| Msg::SwitchTo(Scene::Fragments),>{ "Fragments" }</button>
            { self.view_scene() }
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
            Scene::Crm => {
                html! {
                    <Crm: />
                }
            }
            Scene::CustomComponents => {
                html! {
                    <CustomComponents: />
                }
            }
            Scene::Dashboard => {
                html! {
                    <Dashboard: />
                }
            }
            Scene::Fragments => {
                html! {
                    <Fragments: />
                }
            }
        }
    }
}

fn main() {
    yew::initialize();
    let context = Context {
        console: ConsoleService,
        storage: StorageService::new(Area::Local),
        dialog: DialogService,
        web: FetchService::new(),
        ws: WebSocketService::new(),
    };
    let app: App<_, Scene> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}

