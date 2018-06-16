#![recursion_limit="128"]

#[macro_use]
extern crate log;
extern crate web_logger;
extern crate strum;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate yew;
extern crate counter;
extern crate crm;
extern crate custom_components;
extern crate dashboard;
extern crate fragments;
extern crate game_of_life;
extern crate inner_html;
extern crate large_table;
extern crate mount_point;
extern crate npm_and_rest;
extern crate textarea;
extern crate timer;
extern crate todomvc;
extern crate two_apps;

use strum::IntoEnumIterator;
use std::str::FromStr;
use yew::prelude::*;
use counter::Model as Counter;
use crm::Model as Crm;
use custom_components::Model as CustomComponents;
use dashboard::Model as Dashboard;
use fragments::Model as Fragments;
use game_of_life::Model as GameOfLife;
use inner_html::Model as InnerHtml;
use large_table::Model as LargeTable;
use mount_point::Model as MountPoint;
use npm_and_rest::Model as NpmAndRest;
use textarea::Model as Textarea;
use timer::Model as Timer;
use todomvc::Model as Todomvc;
use two_apps::Model as TwoApps;

#[derive(Debug, Display, EnumString, EnumIter)]
enum Scene {
    NotSelected,
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
    Textarea,
    Timer,
    Todomvc,
    TwoApps,
}

enum Msg {
    SwitchTo(Scene),
}

impl Component for Scene {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Scene::NotSelected
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SwitchTo(scene) => {
                *self = scene;
                true
            }
        }
    }
}

impl Renderable<Scene> for Scene {
    fn view(&self) -> Html<Self> {
        let _options = Scene::iter().map(|scene| {
            html! {
                <option value={ scene.to_string() }, > { scene.to_string() } </option>
            }
        });

        html! {
            <div id="fullscreen",>
                <div id="left_pane",>
                    <h2>{ "Yew showcase" }</h2>
                    <select size="20", value={Scene::NotSelected.to_string()},
                        onchange=|cd| {
                            let scene = match cd {
                                ChangeData::Select(se) => se.value().unwrap(),
                                _ => unreachable!()
                            };
                            match Scene::from_str(&scene) {
                                Ok(scene) => Msg::SwitchTo(scene),
                                _ => unreachable!(),
                            }
                        }
                    , >
                        { for _options }
                    </select>
                </div>
                <div id="right_pane",>
                    <h2>{ self.to_string() }</h2>
                    { self.view_scene() }
                </div>
            </div>
        }
    }
}

impl Scene {
    fn view_scene(&self) -> Html<Self> {
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
            Scene::GameOfLife => {
                html! {
                    <GameOfLife: />
                }
            }
            Scene::InnerHtml => {
                html! {
                    <InnerHtml: />
                }
            }
            Scene::LargeTable => {
                html! {
                    <LargeTable: />
                }
            }
            Scene::MountPoint => {
                html! {
                    <MountPoint: />
                }
            }
            Scene::NpmAndRest => {
                html! {
                    <NpmAndRest: />
                }
            }
            Scene::Textarea => {
                html! {
                    <Textarea: />
                }
            }
            Scene::Timer => {
                html! {
                    <Timer: />
                }
            }
            Scene::Todomvc => {
                html! {
                    <Todomvc: />
                }
            }
            Scene::TwoApps => {
                html! {
                    <TwoApps: />
                }
            }
        }
    }
}

fn main() {
    web_logger::init();
    trace!("Initializing yew...");
    yew::initialize();
    trace!("Creating an application instance...");
    let app: App<Scene> = App::new();
    trace!("Mount the App to the body of the page...");
    app.mount_to_body();
    trace!("Run");
    yew::run_loop();
}

