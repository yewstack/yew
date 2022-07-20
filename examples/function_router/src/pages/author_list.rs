use rand::{distributions, Rng};
use yew::prelude::*;

use crate::components::author_card::AuthorCard;
use crate::components::progress_delay::ProgressDelay;

/// Amount of milliseconds to wait before showing the next set of authors.
const CAROUSEL_DELAY_MS: u32 = 15000;

#[function_component]
pub fn AuthorList() -> Html {
    let seeds = use_state(random_author_seeds);

    let authors = seeds.iter().map(|&seed| {
        html! {
            <div class="tile is-parent">
                <div class="tile is-child">
                    <AuthorCard {seed} />
                </div>
            </div>
        }
    });

    let on_complete = {
        let seeds = seeds.clone();

        Callback::from(move |_| {
            seeds.set(random_author_seeds());
        })
    };

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
                <ProgressDelay duration_ms={CAROUSEL_DELAY_MS} on_complete={on_complete} />
            </div>
        </div>
    }
}

fn random_author_seeds() -> Vec<u32> {
    rand::thread_rng()
        .sample_iter(distributions::Standard)
        .take(2)
        .collect()
}
