use crate::boid::Boid;
use crate::linalg::Vector2D;
use crate::settings::Settings;
use std::time::Duration;
use yew::services::interval::{IntervalService, IntervalTask};
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Debug)]
pub enum Msg {
    Tick,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub settings: Settings,
    pub size: Vector2D,
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
        let boids = (0..props.settings.boids)
            .map(|_| Boid::new_random(props.size, 5.0))
            .collect();

        let interval_task = IntervalService::spawn(
            Duration::from_millis(props.settings.tick_interval_ms),
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
                let Props { settings, size } = &self.props;
                let snapshot = self.boids.clone();
                for boid in &mut self.boids {
                    boid.update(settings, *size, &snapshot);
                }

                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props == self.props {
            false
        } else {
            let settings = &props.settings;
            self.boids
                .resize_with(settings.boids, || Boid::new_random(props.size, 5.0));

            if settings.tick_interval_ms != self.props.settings.tick_interval_ms {
                self.interval_task = IntervalService::spawn(
                    Duration::from_millis(settings.tick_interval_ms),
                    self.link.callback(|_| Msg::Tick),
                );
            }

            true
        }
    }

    fn view(&self) -> Html {
        let size = &self.props.size;
        let view_box = format!("0 0 {} {}", size.x, size.y);

        html! {
            <svg class="simulation-window" viewBox=view_box>
                { for self.boids.iter().map(Boid::render) }
            </svg>
        }
    }
}
