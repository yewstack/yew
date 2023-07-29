use std::cell::RefCell;
use std::rc::Rc;

use gloo::timers::callback::{Interval, Timeout};
use yew::prelude::*;
use yew::{function_component, html};

use crate::components::chessboard::Chessboard;
use crate::components::game_status_board::GameStatusBoard;
use crate::components::score_board::ScoreBoard;
use crate::constant::Status;
use crate::state::{Action, State};

#[function_component]
pub fn App() -> Html {
    let state = use_reducer(State::reset);
    let sec_past = use_state(|| 0_u32);
    let sec_past_timer: Rc<RefCell<Option<Interval>>> = use_mut_ref(|| None);
    let limit_flips_timer: Rc<RefCell<Option<Timeout>>> = use_mut_ref(|| None);
    let sec_past_time = *sec_past;

    use_effect_with(state.clone(), {
        let limit_flips_timer = limit_flips_timer.clone();
        move |state| {
            // game reset
            if state.status == Status::Ready {
                sec_past.set(0);
            }
            // game start
            else if *sec_past == 0 && state.last_card.is_some() {
                let sec_past = sec_past.clone();
                let mut sec = *sec_past;
                *sec_past_timer.borrow_mut() = Some(Interval::new(1000, move || {
                    sec += 1;
                    sec_past.set(sec);
                }));
            }
            // game over
            else if state.status == Status::Passed {
                *sec_past_timer.borrow_mut() = None;
                *limit_flips_timer.borrow_mut() = None;
                state.dispatch(Action::TrySaveBestScore(*sec_past));
            }
            // match failed
            else if state.rollback_cards.is_some() {
                let cloned_state = state.clone();
                let cloned_rollback_cards = state.rollback_cards.clone().unwrap();
                *limit_flips_timer.borrow_mut() = Some(Timeout::new(1000, {
                    let limit_flips_timer = limit_flips_timer.clone();
                    move || {
                        limit_flips_timer.borrow_mut().take();
                        cloned_state.dispatch(Action::RollbackCards(cloned_rollback_cards));
                    }
                }));
            }
            || ()
        }
    });

    let on_reset = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(Action::GameReset))
    };

    let on_flip = {
        let state = state.clone();
        Callback::from(move |card| {
            if limit_flips_timer.borrow().is_some() {
                return;
            }

            *limit_flips_timer.borrow_mut() = Some(Timeout::new(1000, {
                let limit_flips_timer = limit_flips_timer.clone();
                move || {
                    limit_flips_timer.borrow_mut().take();
                }
            }));

            state.dispatch(Action::FlipCard(card));
        })
    };

    html! {
        <div class="game-panel">
            <ScoreBoard unresolved_card_pairs={state.unresolved_card_pairs} best_score={state.best_score} />
            <Chessboard cards={state.cards.clone()} {on_flip} />
            <GameStatusBoard sec_past={sec_past_time} status={state.status} {on_reset}/>
        </div>
    }
}
