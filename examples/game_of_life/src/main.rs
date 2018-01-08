#![recursion_limit="128"]

#[macro_use]
extern crate yew;
extern crate rand;

use yew::html::*;
use std::time::Duration;
use yew::services::Task;
use yew::services::interval::IntervalService;

struct Context {
    interval: IntervalService,
}

#[derive(Clone, Copy, PartialEq)]
enum LifeState {
    Live,
    Dead,
}

#[derive(Clone, Copy)]
struct Cellule {
    life_state: LifeState
}

struct GameOfLife {
    cellules: Vec<Cellule>,
    cellules_width: usize,
    cellules_height: usize,
    job: Option<Box<Task>>,
}

impl Cellule {

    pub fn set_alive(&mut self) { self.life_state = LifeState::Live; }

    pub fn set_dead(&mut self) { self.life_state = LifeState::Dead; }

    pub fn is_alive(self) -> bool { self.life_state == LifeState::Live }

    pub fn count_live_neighbor(&mut self, neighbor: &[Cellule]) -> u32 {
        let mut living : u32 = 0;
        for neighbor in neighbor {
            if neighbor.is_alive() {
                living += 1;
            }
        }
        living
    }

    pub fn is_loneliness(&mut self, neighbors: &[Cellule]) -> bool {
        self.count_live_neighbor(neighbors) < 2
    }

    pub fn is_overpopulation(&mut self, neighbors: &[Cellule]) -> bool {
        self.count_live_neighbor(neighbors) > 3
    }

    pub fn revive(&mut self, neighbors: &[Cellule]) -> bool {
        self.count_live_neighbor(neighbors) == 3
    }

}

impl GameOfLife {

    pub fn random_mutate(&mut self) {
        for cellule in self.cellules.iter_mut() {
            if rand::random() {
                cellule.set_alive();
            } else {
                cellule.set_dead();
            }
        }
    }

    pub fn play(&mut self) {
        self.commute();
    }

    fn reset(&mut self) {
        for cellule in self.cellules.iter_mut() {
            cellule.set_dead();
        }
    }

    pub fn commute(&mut self) {
        let mut not_committed = self.cellules.clone();
        for row in 0..self.cellules_height {
            for col in 0..self.cellules_width {
                let mut neighbors = Vec::new();
                if row>0 && col>0 {
                    neighbors.push(not_committed[self.row_col_as_idx(row-1, col-1)]);
                    neighbors.push(not_committed[self.row_col_as_idx(row, col-1)]);
                    neighbors.push(not_committed[self.row_col_as_idx(row-1, col)]);
                }
                if row<self.cellules_height-1 && col<self.cellules_width-1 {
                    neighbors.push(not_committed[self.row_col_as_idx(row+1, col+1)]);
                    neighbors.push(not_committed[self.row_col_as_idx(row, col+1)]);
                    neighbors.push(not_committed[self.row_col_as_idx(row+1, col)]);
                }
                if row>0 && col<self.cellules_width-1 {
                    neighbors.push(not_committed[self.row_col_as_idx(row-1, col+1)]);
                }
                if row<self.cellules_height-1 && col>0 {
                    neighbors.push(not_committed[self.row_col_as_idx(row+1, col-1)]);
                }

                let current_idx = self.row_col_as_idx(row, col);
                if not_committed[current_idx].is_alive() {
                    if not_committed[current_idx].is_loneliness(&neighbors)
                        || not_committed[current_idx].is_overpopulation(&neighbors) {
                        self.cellules[current_idx].set_dead();
                    }
                } else {
                    if not_committed[current_idx].revive(&neighbors) {
                        self.cellules[current_idx].set_alive();
                    }
                }
            }
        }
    }

    fn row_col_as_idx(&mut self, row: usize, col: usize) -> usize {
        if row>0 { row*self.cellules_width-1 + col } else { row*self.cellules_width + col }
    }

    fn toggle_cellule(&mut self, idx: usize) {
        let mut cellules = self.cellules
            .iter_mut()
            .collect::<Vec<_>>();
        let cellule = cellules.get_mut(idx).unwrap();
        cellule.life_state = if cellule.life_state == LifeState::Live { LifeState::Dead } else { LifeState::Live };
    }
}


enum Msg {
    Random,
    Start,
    Step,
    Reset,
    Stop,
    ToggleCellule(usize)
}

impl Component<Context> for GameOfLife {
    type Msg = Msg;

    fn create(_: &mut ScopeRef<Context, Msg>) -> Self {
        GameOfLife {
            cellules: vec![Cellule { life_state: LifeState::Dead }; 2000],
            cellules_width: 50,
            cellules_height: 40,
            job : None
        }
    }

    fn update(&mut self, msg: Msg, context: &mut ScopeRef<Context, Msg>) {
        match msg {
            Msg::Random => {
                self.random_mutate();
                println!("Random");
            },
            Msg::Start => {
                let callback = context.send_back(|_| Msg::Step);
                let handle = context.interval.spawn(Duration::from_millis(200), callback);
                self.job = Some(Box::new(handle));
                println!("Start");
            },
            Msg::Step => {
                self.play();
            },
            Msg::Reset => {
                self.reset();
                println!("Reset");
            },
            Msg::Stop => {
                if let Some(mut task) = self.job.take() {
                    task.cancel();
                }
                self.job = None;
                println!("Stop");
            },
            Msg::ToggleCellule(idx) => {
                self.toggle_cellule(idx);
            }
        }
    }

    fn view(&self) -> Html<Context, Msg> {
        html! {
            <div>
                <section class="game-container",>
                    <header class="app-header",>
                        <img src="favicon.ico", class="app-logo",/>
                        <h1 class="app-title",>{ "Game of Life" }</h1>
                    </header>
                    <section class="game-area",>
                        <div class="game-of-life",>
                            { for self.cellules.iter().enumerate().map(view_cellule) }
                        </div>
                        <div class="game-buttons",>
                            <button class="game-button", onclick=move|_| Msg::Random,>{ "Random" }</button>
                            <button class="game-button", onclick=move|_| Msg::Step,>{ "Step" }</button>
                            <button class="game-button", onclick=move|_| Msg::Start,>{ "Start" }</button>
                            <button class="game-button", onclick=move|_| Msg::Stop,>{ "Stop" }</button>
                            <button class="game-button", onclick=move|_| Msg::Reset,>{ "Reset" }</button>
                        </div>
                    </section>
                </section>
                <footer class="app-footer",>
                    <strong class="footer-text",>
                      { "Game of Life - a yew experiment " }
                    </strong>
                    <a href="https://github.com/DenisKolodin/yew", target="_blank",>{ "source" }</a>
                </footer>
            </div>
        }
    }
}

fn view_cellule((idx, cellule): (usize, &Cellule)) -> Html<Context, Msg> {
    html! {
        <div class=("game-cellule", if cellule.life_state == LifeState::Live { "cellule-live" } else { "cellule-dead" }),
            onclick=move |_| Msg::ToggleCellule(idx),> </div>
    }
}

fn main() {
    yew::initialize();
    let context = Context {
        interval: IntervalService::new(),
    };
    let app = Scope::new(context);
    app.mount_to_body::<GameOfLife>();
    yew::run_loop();
}
