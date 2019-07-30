#![recursion_limit = "128"]

use counter::Model as Counter;
use crm::Model as Crm;
use custom_components::Model as CustomComponents;
use dashboard::Model as Dashboard;
use fragments::Model as Fragments;
use game_of_life::Model as GameOfLife;
use inner_html::Model as InnerHtml;
use large_table::Model as LargeTable;
use log::trace;
use mount_point::Model as MountPoint;
use npm_and_rest::Model as NpmAndRest;
use routing::Model as Routing;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};
use textarea::Model as Textarea;
use timer::Model as Timer;
use todomvc::Model as Todomvc;
use two_apps::Model as TwoApps;
use yew::components::Select;
use yew::{html, App, Component, ComponentLink, Html, Renderable, ShouldRender};

#[derive(Clone, Debug, Display, EnumString, EnumIter, PartialEq)]
enum Scene {
    Counter,
    Crm,
    CustomComponents,
    Dashboard,
    Fragments,
    GameOfLife,
    InnerHtml,
    LargeTable,
    MountPoint,
    NpmAndRest,
    Routing,
    Textarea,
    Timer,
    Todomvc,
    TwoApps,
}

struct Model {
    scene: Option<Scene>,
}

enum Msg {
    SwitchTo(Scene),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { scene: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SwitchTo(scene) => {
                self.scene = Some(scene);
                true
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div id="fullscreen">
                <div id="left_pane">
                    <h2>{ "Yew showcase" }</h2>
                    <Select<Scene>
                        selected=self.scene.clone()
                        options=Scene::iter().collect::<Vec<_>>()
                        onchange=Msg::SwitchTo />
                </div>
                <div id="right_pane">
                    { self.view_scene() }
                </div>
            </div>
        }
    }
}

impl Model {
    fn view_scene(&self) -> Html<Self> {
        if let Some(scene) = self.scene.as_ref() {
            match scene {
                Scene::Counter => html! { <Counter /> },
                Scene::Crm => html! { <Crm /> },
                Scene::CustomComponents => html! { <CustomComponents /> },
                Scene::Dashboard => html! { <Dashboard /> },
                Scene::Fragments => html! { <Fragments /> },
                Scene::GameOfLife => html! { <GameOfLife /> },
                Scene::InnerHtml => html! { <InnerHtml /> },
                Scene::LargeTable => html! { <LargeTable /> },
                Scene::MountPoint => html! { <MountPoint /> },
                Scene::NpmAndRest => html! { <NpmAndRest /> },
                Scene::Routing => html! { <Routing /> },
                Scene::Textarea => html! { <Textarea /> },
                Scene::Timer => html! { <Timer /> },
                Scene::Todomvc => html! { <Todomvc /> },
                Scene::TwoApps => html! { <TwoApps /> },
            }
        } else {
            html! {
                <p>{ "Select the scene, please." }</p>
            }
        }
    }
}

fn main() {
    web_logger::init();
    trace!("Initializing yew...");
    yew::initialize();
    trace!("Creating an application instance...");
    let app: App<Model> = App::new();
    trace!("Mount the App to the body of the page...");
    app.mount_to_body();
    trace!("Run");
    yew::run_loop();
}
