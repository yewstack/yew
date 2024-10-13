use gloo::timers::callback::Interval;
use yew::html::Scope;
use yew::{classes, html, Component, Context, Html};

mod conway;

pub enum Msg {
    Random,
    Start,
    Step,
    Reset,
    Stop,
    ToggleCellule((usize, usize)),
    Tick,
}

pub struct App {
    active: bool,
    conway: conway::Conway,
    _interval: Interval,
}

impl App {
    fn view_cellule(&self, row: usize, col: usize, link: &Scope<Self>) -> Html {
        let status = if self.conway.alive(row, col) {
            "cellule-live"
        } else {
            "cellule-dead"
        };
        html! {
            <div class={classes!("game-cellule", status)}
                onclick={link.callback(move |_| Msg::ToggleCellule((row,col)))}>
            </div>
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|_| Msg::Tick);

        Self {
            active: false,
            conway: conway::Conway::new(53, 40),
            _interval: Interval::new(200, move || callback.emit(())),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let mut render = true;
        match msg {
            Msg::Random => {
                self.conway.random_mutate();
                log::info!("Random");
            }
            Msg::Start => {
                self.active = true;
                log::info!("Start");
                render = false;
            }
            Msg::Step => {
                self.conway.step();
            }
            Msg::Reset => {
                self.conway.reset();
                log::info!("Reset");
            }
            Msg::Stop => {
                self.active = false;
                log::info!("Stop");
                render = false;
            }
            Msg::ToggleCellule((row, col)) => self.conway.toggle(row, col),
            Msg::Tick => {
                if self.active {
                    self.conway.step();
                } else {
                    render = false;
                }
            }
        }
        render
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cell_rows = self
            .conway
            .cellules
            .chunks(self.conway.width)
            .enumerate()
            .map(|(row, cellules)| {
                let cells = cellules
                    .iter()
                    .enumerate()
                    .map(|(col, _)| self.view_cellule(row, col, ctx.link()));
                html! {
                    <div class="game-row">
                        { for cells }
                    </div>
                }
            });

        html! {
            <div>
                <section class="game-container">
                    <header class="app-header">
                        <img alt="The app logo" src="favicon.ico" class="app-logo"/>
                        <h1 class="app-title">{ "Game of Life" }</h1>
                    </header>
                    <section class="game-area">
                        <div class="game-of-life">
                            { for cell_rows }
                        </div>
                        <div class="game-buttons">
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Random)}>{ "Random" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Step)}>{ "Step" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Start)}>{ "Start" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Stop)}>{ "Stop" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Reset)}>{ "Reset" }</button>
                        </div>
                    </section>
                </section>
                <footer class="app-footer">
                    <strong class="footer-text">
                      { "Game of Life - a yew experiment " }
                    </strong>
                    <a href="https://github.com/yewstack/yew" target="_blank">{ "source" }</a>
                </footer>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::trace!("Initializing yew...");
    yew::Renderer::<App>::new().render();
}
