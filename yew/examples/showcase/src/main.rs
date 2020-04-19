#![recursion_limit = "128"]

cfg_if::cfg_if! {
    if #[cfg(feature = "std_web")] {
        use counter_std_web as counter;
        use inner_html_std_web as inner_html;
        use mount_point_std_web as mount_point;
        use node_refs_std_web as node_refs;
        use npm_and_rest_std_web as npm_and_rest;
        use todomvc_std_web as todomvc;
        use two_apps_std_web as two_apps;
    } else if #[cfg(feature = "web_sys")] {
        use counter_web_sys as counter;
        use inner_html_web_sys as inner_html;
        use mount_point_web_sys as mount_point;
        use node_refs_web_sys as node_refs;
        use npm_and_rest_web_sys as npm_and_rest;
        use todomvc_web_sys as todomvc;
        use two_apps_web_sys as two_apps;
    }
}

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
use node_refs::Model as NodeRefs;
use npm_and_rest::Model as NpmAndRest;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};
use textarea::Model as Textarea;
use timer::Model as Timer;
use todomvc::Model as Todomvc;
use two_apps::TwoModels as TwoApps;
use yew::components::Select;
use yew::{html, App, Component, ComponentLink, Html, ShouldRender};

#[derive(Clone, Debug, Display, EnumString, EnumIter, PartialEq)]
enum Scene {
    Counter,
    Crm,
    CustomComponents,
    Dashboard,
    NodeRefs,
    Fragments,
    GameOfLife,
    InnerHtml,
    LargeTable,
    MountPoint,
    NpmAndRest,
    Textarea,
    Timer,
    Todomvc,
    TwoApps,
}

struct Model {
    scene: Option<Scene>,
    link: ComponentLink<Self>,
}

enum Msg {
    SwitchTo(Scene),
    Reset,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { scene: None, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SwitchTo(scene) => {
                self.scene = Some(scene);
                true
            }
            Msg::Reset => {
                self.scene = None;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div id="fullscreen">
                <style>{ self.view_style() }</style>
                <div id="left_pane">
                    <h2>{ "Yew showcase" }</h2>
                    <Select<Scene>
                        selected=self.scene.clone()
                        options=Scene::iter().collect::<Vec<_>>()
                        onchange=self.link.callback(Msg::SwitchTo) />
                    <button onclick=self.link.callback(|_| Msg::Reset)>
                        { "Reset" }
                    </button>
                </div>
                <div id="right_pane">
                    { self.view_scene() }
                </div>
            </div>
        }
    }
}

impl Model {
    fn view_scene(&self) -> Html {
        if let Some(scene) = self.scene.as_ref() {
            match scene {
                Scene::Counter => html! { <Counter /> },
                Scene::Crm => html! { <Crm /> },
                Scene::CustomComponents => html! { <CustomComponents /> },
                Scene::Dashboard => html! { <Dashboard /> },
                Scene::NodeRefs => html! { <NodeRefs /> },
                Scene::Fragments => html! { <Fragments /> },
                Scene::GameOfLife => html! { <GameOfLife /> },
                Scene::InnerHtml => html! { <InnerHtml /> },
                Scene::LargeTable => html! { <LargeTable /> },
                Scene::MountPoint => html! { <MountPoint /> },
                Scene::NpmAndRest => html! { <NpmAndRest /> },
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

    fn view_style(&self) -> &str {
        if let Some(scene) = self.scene.as_ref() {
            match scene {
                Scene::GameOfLife => include_str!("../../game_of_life/static/styles.css"),
                Scene::LargeTable => include_str!("../../large_table/static/styles.css"),
                Scene::Todomvc => include_str!("../static/todomvc.css"),
                _ => "",
            }
        } else {
            ""
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
