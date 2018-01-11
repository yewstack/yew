#![recursion_limit="128"]

#[macro_use]
extern crate yew;
extern crate rand;

use yew::prelude::*;
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

fn update(context: &mut AppContext<Context, GameOfLife, Msg>, gof: &mut GameOfLife, msg: Msg) -> ShouldRender {
    match msg {
        Msg::Random => {
            gof.random_mutate();
            println!("Random");
        },
        Msg::Start => {
            let callback = context.send_back(|_| Msg::Step);
            let handle = context.interval.spawn(Duration::from_millis(200), callback);
            gof.job = Some(Box::new(handle));
            println!("Start");
        },
        Msg::Step => {
            gof.play();
        },
        Msg::Reset => {
            gof.reset();
            println!("Reset");
        },
        Msg::Stop => {
            if let Some(mut task) = gof.job.take() {
                task.cancel();
            }
            gof.job = None;
            println!("Stop");
        },
        Msg::ToggleCellule(idx) => {
            gof.toggle_cellule(idx);
        }
    }
    true
}

fn view(gof: &GameOfLife) -> AppHtml<Context, GameOfLife, Msg> {
    html! {
        <div>
            <section class="game-container",>
                <header class="app-header",>
                    <img src="favicon.ico", class="app-logo",/>
                    <h1 class="app-title",>{ "Game of Life" }</h1>
                </header>
                <section class="game-area",>
                    <div class="game-of-life",>
                        { for gof.cellules.iter().enumerate().map(view_cellule) }
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

fn view_cellule((idx, cellule): (usize, &Cellule)) -> AppHtml<Context, GameOfLife, Msg> {
    html! {
        <div class=("game-cellule", if cellule.life_state == LifeState::Live { "cellule-live" } else { "cellule-dead" }),
            onclick=move |_| Msg::ToggleCellule(idx),> </div>
    }
}

fn main() {
    yew::initialize();
    let app = App::new();
    let context = Context {
        interval: IntervalService::new(),
    };
    let gof = GameOfLife {
        cellules: vec![Cellule { life_state: LifeState::Dead }; 2000],
        cellules_width: 50,
        cellules_height: 40,
        job : None
    };
    app.mount(context, gof, update, view);
    yew::run_loop();
}
