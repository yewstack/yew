use crate::boid::Boid;
use crate::math::Vector2D;
use crate::settings::Settings;
use std::time::Duration;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_services::interval::{IntervalService, IntervalTask};

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
    props: Props,
    link: ComponentLink<Self>,
    boids: Vec<Boid>,
    interval_task: IntervalTask,
}
impl Component for Simulation {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let settings = &props.settings;
        let boids = (0..settings.boids)
            .map(|_| Boid::new_random(settings))
            .collect();

        let interval_task = IntervalService::spawn(
            Duration::from_millis(settings.tick_interval_ms),
            link.callback(|_| Msg::Tick),
        );

        Self {
            props,
            link,
            boids,
            interval_task,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Tick => {
                let Props {
                    ref settings,
                    paused,
                    ..
                } = self.props;

                if paused {
                    false
                } else {
                    Boid::update_all(settings, &mut self.boids);
                    true
                }
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props == self.props {
            false
        } else {
            if props.generation != self.props.generation {
                // generation changed; restart from scratch.
                self.boids.clear();
            }

            let settings = &props.settings;
            self.boids
                .resize_with(settings.boids, || Boid::new_random(settings));

            if settings.tick_interval_ms != self.props.settings.tick_interval_ms {
                // as soon as the previous task is dropped it is cancelled.
                // We don't need to worry about manually stopping it.
                self.interval_task = IntervalService::spawn(
                    Duration::from_millis(settings.tick_interval_ms),
                    self.link.callback(|_| Msg::Tick),
                );
            }

            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let view_box = format!("0 0 {} {}", SIZE.x, SIZE.y);

        html! {
            <svg class="simulation-window" viewBox=view_box>
                { for self.boids.iter().map(Boid::render) }
            </svg>
        }
    }
}
