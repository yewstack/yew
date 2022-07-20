use settings::Settings;
use simulation::Simulation;
use slider::Slider;
use yew::html::Scope;
use yew::{html, Component, Context, Html};

mod boid;
mod math;
mod settings;
mod simulation;
mod slider;

pub enum Msg {
    ChangeSettings(Settings),
    ResetSettings,
    RestartSimulation,
    TogglePause,
}

pub struct App {
    settings: Settings,
    generation: usize,
    paused: bool,
}
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            settings: Settings::load(),
            generation: 0,
            paused: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Msg) -> bool {
        match msg {
            Msg::ChangeSettings(settings) => {
                self.settings = settings;
                self.settings.store();
                true
            }
            Msg::ResetSettings => {
                self.settings = Settings::default();
                Settings::remove();
                true
            }
            Msg::RestartSimulation => {
                self.generation = self.generation.wrapping_add(1);
                true
            }
            Msg::TogglePause => {
                self.paused = !self.paused;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self {
            ref settings,
            generation,
            paused,
            ..
        } = *self;

        html! {
            <>
                <h1 class="title">{ "Boids" }</h1>
                <Simulation settings={settings.clone()} {generation} {paused} />
                { self.view_panel(ctx.link()) }
            </>
        }
    }
}
impl App {
    fn view_panel(&self, link: &Scope<Self>) -> Html {
        let pause_text = if self.paused { "Resume" } else { "Pause" };
        html! {
            <div class="panel">
                { self.view_settings(link) }
                <div class="panel__buttons">
                    <button onclick={link.callback(|_| Msg::TogglePause)}>{ pause_text }</button>
                    <button onclick={link.callback(|_| Msg::ResetSettings)}>{ "Use Defaults" }</button>
                    <button onclick={link.callback(|_| Msg::RestartSimulation)}>{ "Restart" }</button>
                </div>
            </div>
        }
    }

    fn view_settings(&self, link: &Scope<Self>) -> Html {
        let Self { settings, .. } = self;

        // This helper macro creates a callback which applies the new value to the current settings
        // and sends `Msg::ChangeSettings`. Thanks to this, we don't need to have
        // "ChangeBoids", "ChangeCohesion", etc. messages, but it comes at the cost of
        // cloning the `Settings` struct each time.
        macro_rules! settings_callback {
            ($link:expr, $settings:ident; $key:ident as $ty:ty) => {{
                let settings = $settings.clone();
                $link.callback(move |value| {
                    let mut settings = settings.clone();
                    settings.$key = value as $ty;
                    Msg::ChangeSettings(settings)
                })
            }};
            ($link:expr, $settings:ident; $key:ident) => {
                settings_callback!($link, $settings; $key as f64)
            }
        }

        html! {
            <div class="settings">
                <Slider label="Number of Boids"
                    min=1.0 max=600.0
                    onchange={settings_callback!(link, settings; boids as usize)}
                    value={settings.boids as f64}
                />
                <Slider label="View Distance"
                    max=500.0 step=10.0
                    onchange={settings_callback!(link, settings; visible_range)}
                    value={settings.visible_range}
                />
                <Slider label="Spacing"
                    max=100.0
                    onchange={settings_callback!(link, settings; min_distance)}
                    value={settings.min_distance}
                />
                <Slider label="Max Speed"
                    max=50.0
                    onchange={settings_callback!(link, settings; max_speed)}
                    value={settings.max_speed}
                />
                <Slider label="Cohesion"
                    max=0.5 percentage=true
                    onchange={settings_callback!(link, settings; cohesion_factor)}
                    value={settings.cohesion_factor}
                />
                <Slider label="Separation"
                    max=1.0 percentage=true
                    onchange={settings_callback!(link, settings; separation_factor)}
                    value={settings.separation_factor}
                />
                <Slider label="Alignment"
                    max=0.5 percentage=true
                    onchange={settings_callback!(link, settings; alignment_factor)}
                    value={settings.alignment_factor}
                />
                <Slider label="Turn Speed"
                    max=1.5 percentage=true
                    onchange={settings_callback!(link, settings; turn_speed_ratio)}
                    value={settings.turn_speed_ratio}
                />
                <Slider label="Color Adaption"
                    max=1.5 percentage=true
                    onchange={settings_callback!(link, settings; color_adapt_factor)}
                    value={settings.color_adapt_factor}
                />
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
