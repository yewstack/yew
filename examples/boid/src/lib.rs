mod vector;
mod rand;
mod boid;
mod triangle;

use yew::prelude::{ ComponentLink, Component, html, ShouldRender, Html };
use crate::rand::Rand;
use crate::boid::Boid;
use crate::triangle::Triangle;

pub struct Model {
    boids: Vec<Boid>,
}

const WIDTH: u64 = 600;
const HEIGHT: u64 = 400;

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let mut rng = Rand::new();
        Self {
            boids: (0..100).map(|_| Boid::new(&mut rng)).collect(),
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let boids = (&self.boids).into_iter().map(|boid| boid2triangle(&boid));
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
