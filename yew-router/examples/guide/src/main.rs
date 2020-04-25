use yew::prelude::*;

use guide::{Guide, Page};
use yew::virtual_dom::VNode;

fn main() {
    yew::initialize();
    //    web_logger::init();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}

pub struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> VNode {
        html! {
            <Guide>
                <Page
                    uri="https://raw.githubusercontent.com/hgzimmerman/YewRouter/master/examples/guide/chapters/01_intro.md"
                    page_url="/intro"
                    title="Intro"
                />
                <Page
                    uri="https://raw.githubusercontent.com/hgzimmerman/YewRouter/master/examples/guide/chapters/02_router_component.md"
                    page_url="/router"
                    title="Router Component"
                />
                <Page
                    uri="https://raw.githubusercontent.com/hgzimmerman/YewRouter/master/examples/guide/chapters/03_route_macro.md"
                    page_url="/route_macro"
                    title="Route Macro"
                />
                <Page
                     uri="https://raw.githubusercontent.com/hgzimmerman/YewRouter/master/examples/guide/chapters/04_render.md"
                     page_url="/render"
                     title="Render"
                />
                <Page
                    uri="https://raw.githubusercontent.com/hgzimmerman/YewRouter/master/examples/guide/chapters/05_testing.md"
                    page_url="/testing"
                    title="Testing"
                />
            </Guide>
        }
    }
}
