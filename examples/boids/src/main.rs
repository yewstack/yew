use linalg::Vector2D;
use settings::Settings;
use simulation::Simulation;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

mod boid;
mod linalg;
mod settings;
mod simulation;

const SIZE: Vector2D = Vector2D::new(2000.0, 1600.0);

pub enum Msg {}

pub struct Model {
    settings: Settings,
}
impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            settings: settings::load(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {}
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        unimplemented!()
    }

    fn view(&self) -> Html {
        html! {
            <Simulation size=SIZE settings=self.settings.clone() />
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
