use gloo::utils::document;
use slab::Slab;
use web_sys::Element;
use yew::prelude::*;

mod counter;

use counter::{CounterModel, CounterProps};

// Define the possible messages which can be sent to the component
pub enum Msg {
    // Spawns a new instance of the CounterModel app
    SpawnCounterAppInstance,
    // Destroys an instance of a CounterModel app
    DestroyCounterApp(usize),
}

pub struct App {
    apps: Slab<(Element, AppHandle<CounterModel>)>, /* Contains the spawned apps and their
                                                     * parent div elements */
    apps_container_ref: NodeRef,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            apps: Slab::new(),
            apps_container_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let app_container = self
            .apps_container_ref
            .cast::<Element>()
            .expect("Failed to cast app container div to HTMLElement");

        match msg {
            Msg::SpawnCounterAppInstance => {
                // Create a new <div> HtmlElement where the new app will live
                let app_div = document()
                    .create_element("div")
                    .expect("Failed to create <div> element");

                // Append the div to the document body
                let _ = app_container
                    .append_child(&app_div)
                    .expect("Failed to append app div app container div");

                // Reserve an entry for the new app
                let app_entry = self.apps.vacant_entry();

                // Get the key for the entry and create and mount a new CounterModel app
                // with a callback that destroys the app when emitted
                let app_key = app_entry.key();
                let new_counter_app = yew::Renderer::<CounterModel>::with_root_and_props(
                    app_div.clone(),
                    CounterProps {
                        destroy_callback: ctx
                            .link()
                            .callback(move |_| Msg::DestroyCounterApp(app_key)),
                    },
                )
                .render();

                // Insert the app and the app div to our app collection
                app_entry.insert((app_div, new_counter_app));
            }
            Msg::DestroyCounterApp(app_id) => {
                // Get the app from the app slabmap
                let (app_div, app) = self.apps.remove(app_id);

                // Destroy the app
                app.destroy();

                // Remove the app div from the DOM
                app_div.remove()
            }
        }

        // Never render
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // We will only render once, and then do the rest of the DOM changes
        // by mounting/destroying appinstances of CounterModel
        html! {
            <>
                <div class="panel">
                    // Create button to create a new app
                    <button
                        class="create"
                        onclick={ctx.link().callback(|_| Msg::SpawnCounterAppInstance)}
                    >
                        { "Spawn new CounterModel app" }
                    </button>
                </div>
                // Create a container for all the app instances
                <div ref={self.apps_container_ref.clone()}>
                </div>
            </>
        }
    }
}

fn main() {
    // Start main app
    yew::Renderer::<App>::new().render();
}
