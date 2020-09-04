mod boid;
mod vector;

use crate::boid::Boid;
use crate::vector::Vector;
use std::time::Duration;
use std::f64::consts::PI;
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
    let points = get_points_str(&boid.position, &boid.velocity);
    html! {
        <polygon points=points />
    }
}

fn get_points_str(position: &Vector, velocity: &Vector) -> String {
    let direction = velocity.y.atan2(velocity.x);
    let size = 10.0;
    let convert_position = |i: usize| {
        (
            position.x + size * (direction + ((i as f64) * 2.0 * PI / 3.0)).cos(),
            position.y + size * (direction + ((i as f64) * 2.0 * PI / 3.0)).sin(),
        )
    };
    (0..3)
        .map(convert_position)
        .map(|(x, y): (f64, f64)| format!("{},{}", x, y))
        .collect::<Vec<String>>()
        .join(" ")
}
