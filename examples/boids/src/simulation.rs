use std::fmt::Write;

use gloo::timers::callback::Interval;
use yew::{Component, Context, Html, Properties, html};

use crate::boid::Boid;
use crate::math::{self, Vector2D};
use crate::settings::Settings;

pub const SIZE: Vector2D = Vector2D::new(1600.0, 1000.0);

#[derive(Debug)]
pub enum Msg {
    Tick,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub settings: Settings,
    #[prop_or_default]
    pub generation: usize,
    #[prop_or_default]
    pub paused: bool,
}

#[derive(Debug)]
pub struct Simulation {
    boids: Vec<Boid>,
    interval: Interval,
    generation: usize,
}
impl Component for Simulation {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let settings = ctx.props().settings.clone();
        let boids = (0..settings.boids)
            .map(|_| Boid::new_random(&settings))
            .collect();

        let interval = {
            let link = ctx.link().clone();
            Interval::new(settings.tick_interval_ms as u32, move || {
                link.send_message(Msg::Tick)
            })
        };

        let generation = ctx.props().generation;
        Self {
            boids,
            interval,
            generation,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick => {
                let Props {
                    ref settings,
                    paused,
                    ..
                } = *ctx.props();

                if paused {
                    false
                } else {
                    Boid::update_all(settings, &mut self.boids);
                    true
                }
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        let props = ctx.props();
        let should_reset =
            old_props.settings != props.settings || self.generation != props.generation;
        self.generation = props.generation;
        if should_reset {
            self.boids.clear();

            let settings = &props.settings;
            self.boids
                .resize_with(settings.boids, || Boid::new_random(settings));

            // as soon as the previous task is dropped it is cancelled.
            // We don't need to worry about manually stopping it.
            self.interval = {
                let link = ctx.link().clone();
                Interval::new(settings.tick_interval_ms as u32, move || {
                    link.send_message(Msg::Tick)
                })
            };

            true
        } else {
            false
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let view_box = format!("0 0 {} {}", SIZE.x, SIZE.y);

        html! {
            <svg class="simulation-window" viewBox={view_box}>
                for boid in &self.boids {

                    let color = format!("hsl({:.3}rad, 100%, 50%)", boid.hue);
                    let mut points = String::new();
                    for offset in [
                        (0. * math::FRAC_TAU_3, 2.0),
                        (1. * math::FRAC_TAU_3, 1.0),
                        (2. * math::FRAC_TAU_3, 1.0),
                    ]
                            .iter()
                            .copied()
                            .map(move |(angle, radius_mul)| Vector2D::from_polar(angle + boid.velocity.angle(), radius_mul * boid.radius)) {
                        let Vector2D { x, y } = boid.position + offset;
                        // Write to string will never fail.
                        let _ = write!(points, "{x:.2},{y:.2} ");
                    }

                    <polygon {points} fill={color} />
                }
            </svg>
        }
    }
}
