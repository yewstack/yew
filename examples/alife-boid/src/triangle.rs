use yew::prelude::{ ComponentLink, Component, html, ShouldRender, Html, Properties };
use crate::vector::Vector;
use std::f64::consts::PI;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub velocity: Vector,
    pub position: Vector,
}

pub struct Triangle {
    velocity: Vector,
    position: Vector,
}

impl Component for Triangle {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            velocity: props.velocity,
            position: props.position,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, prop: Self::Properties) -> ShouldRender {
        if prop.velocity == self.velocity && prop.position == self.velocity {
            return false;
        }

        if prop.velocity != self.velocity {
            self.velocity = prop.velocity.clone();
        }

        if prop.position != self.position {
            self.position = prop.position.clone();
        }
        true
    }

    fn view(&self) -> Html {
        let points = get_points_str(&self.position, &self.velocity);
        html! {
            <polygon points=points />
        }
    }
}

fn get_points_str(position: &Vector, velocity: &Vector) -> String {
    let direction = velocity.y.atan2(velocity.x);
    let size = 10.0;
    let convert_position = |i: usize| (position.x + size * (direction + ((i as f64) * 2.0 * PI / 3.0)).cos(), position.y + size * (direction + ((i as f64) * 2.0 * PI / 3.0)).sin());
    (0..3)
        .map(convert_position)
        .map(|(x, y): (f64, f64)| format!("{},{}", x, y))
        .collect::<Vec<String>>()
        .join(" ")
}

