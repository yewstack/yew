use crate::boid::Boid;
use crate::math::Vector2D;
use crate::settings::Settings;
use std::time::Duration;
use yew::component::{Component, Context};
use yew::services::interval::{IntervalService, IntervalTask};
use yew::{html, Html, Properties, ShouldRender};

pub const SIZE: Vector2D = Vector2D::new(1600.0, 1000.0);

#[derive(Debug)]
pub enum Msg {
    Tick,
}

#[derive(Debug, PartialEq, Properties)]
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
    interval_task: IntervalTask,
}

impl Component for Simulation {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let settings = &ctx.props.settings;
        let boids = (0..settings.amount_of_boids)
            .map(|_| Boid::new_random(settings))
            .collect();

        let interval_task = IntervalService::spawn(
            Duration::from_millis(settings.tick_interval_ms),
            ctx.callback(|_| Msg::Tick),
        );

        Self {
            boids,
            interval_task,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Tick => {
                if ctx.props.paused {
                    false
                } else {
                    Boid::update_all(&ctx.props.settings, &mut self.boids);
                    true
                }
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, new_props: &Self::Properties) -> ShouldRender {
        if ctx.props.generation != new_props.generation {
            // generation changed; restart from scratch.
            self.boids.clear();
        }

        let new_settings = &new_props.settings;
        self.boids.resize_with(new_settings.amount_of_boids, || {
            Boid::new_random(new_settings)
        });

        if ctx.props.settings.tick_interval_ms != new_settings.tick_interval_ms {
            // as soon as the previous task is dropped it is cancelled.
            // We don't need to worry about manually stopping it.
            self.interval_task = IntervalService::spawn(
                Duration::from_millis(new_settings.tick_interval_ms),
                ctx.callback(|_| Msg::Tick),
            );
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let view_box = format!("0 0 {} {}", SIZE.x, SIZE.y);

        html! {
            <svg class="simulation-window" viewBox=view_box>
                { for self.boids.iter().map(Boid::render) }
            </svg>
        }
    }
}
