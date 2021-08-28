use crate::boid::Boid;
use crate::math::Vector2D;
use crate::settings::Settings;
use gloo::timers::callback::Interval;
use yew::{html, Component, Context, Html, Properties};

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
}
impl Component for Simulation {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let settings = &ctx.props().settings;
        let boids = (0..settings.boids)
            .map(|_| Boid::new_random(settings))
            .collect();

        let interval = {
            let link = ctx.link().clone();
            Interval::new(settings.tick_interval_ms as u32, move || {
                link.send_message(Msg::Tick)
            })
        };

        Self { boids, interval }
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

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.boids.clear();

        let settings = &ctx.props().settings;
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
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let view_box = format!("0 0 {} {}", SIZE.x, SIZE.y);

        html! {
            <svg class="simulation-window" viewBox={view_box}>
                { for self.boids.iter().map(Boid::render) }
            </svg>
        }
    }
}
