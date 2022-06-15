use rand::{distributions, Rng};
use yew::prelude::*;

use crate::components::author_card::AuthorCard;
use crate::components::progress_delay::ProgressDelay;

/// Amount of milliseconds to wait before showing the next set of authors.
const CAROUSEL_DELAY_MS: u64 = 15000;

pub enum Msg {
    NextAuthors,
}

pub struct AuthorList {
    seeds: Vec<u64>,
}
impl Component for AuthorList {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            seeds: random_author_seeds(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NextAuthors => {
                self.seeds = random_author_seeds();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let authors = self.seeds.iter().map(|&seed| {
            html! {
                <div class="tile is-parent">
                    <div class="tile is-child">
                        <AuthorCard {seed} />
                    </div>
                </div>
            }
        });

        html! {
            <div class="container">
                <section class="hero">
                    <div class="hero-body">
                        <div class="container">
                            <h1 class="title">{ "Authors" }</h1>
                            <h2 class="subtitle">
                                { "Meet the definitely real people behind your favourite Yew content" }
                            </h2>
                        </div>
                    </div>
                </section>
                <p class="section py-0">
                    { "It wouldn't be fair " }
                    <i>{ "(or possible :P)" }</i>
                    {" to list each and every author in alphabetical order."}
                    <br />
                    { "So instead we chose to put more focus on the individuals by introducing you to two people at a time" }
                </p>
                <div class="section">
                    <div class="tile is-ancestor">
                        { for authors }
                    </div>
                    <ProgressDelay duration_ms={CAROUSEL_DELAY_MS} on_complete={ctx.link().callback(|_| Msg::NextAuthors)} />
                </div>
            </div>
        }
    }
}

fn random_author_seeds() -> Vec<u64> {
    rand::thread_rng()
        .sample_iter(distributions::Standard)
        .take(2)
        .collect()
}
