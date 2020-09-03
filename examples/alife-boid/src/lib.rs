mod boid;
mod triangle;
mod vector;

use crate::boid::Boid;
use crate::triangle::Triangle;
use std::time::Duration;
use rand::prelude::thread_rng;
use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};
use yew::services::{IntervalService, Task};

pub struct Model {
    boids: Vec<Boid>,
    #[allow(unused)]
    job: Box<dyn Task>,
}

pub enum Msg {
    Tick,
}

const WIDTH: u64 = 600;
const HEIGHT: u64 = 400;

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_| Msg::Tick);
        let handle = IntervalService::spawn(Duration::from_millis(50), callback);

        let mut rng = thread_rng();
        Self {
            boids: (0..100).map(|_| Boid::new(&mut rng)).collect(),
            job: Box::new(handle),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Tick => {
                let boids = self.boids.clone();
                for boid in &mut self.boids {
                    boid.next_state(&boids);
                }
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let boids = (&self.boids).iter().map(|boid| boid2triangle(&boid));
        html! {
            <svg width=WIDTH height=HEIGHT viewBox={ format!("0 0 {} {}", WIDTH, HEIGHT) } xmlns={ "http://www.w3.org/2000/svg" }>
                { for boids }
            </svg>
        }
    }
}

fn boid2triangle(boid: &Boid) -> Html {
    html! {
        <Triangle position=boid.position.clone() velocity=boid.velocity.clone() />
    }
}
