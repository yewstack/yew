use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::pagination::{PageQuery, Pagination};
use crate::components::post_card::PostCard;
use crate::Route;

const ITEMS_PER_PAGE: u32 = 10;
const TOTAL_PAGES: u32 = u32::MAX / ITEMS_PER_PAGE;

#[function_component]
pub fn PostList() -> Html {
    let location = use_location().unwrap();
    let current_page = location.query::<PageQuery>().map(|it| it.page).unwrap_or(1);

    let posts = {
        let start_seed = (current_page - 1) * ITEMS_PER_PAGE;
        let mut cards = (0..ITEMS_PER_PAGE).map(|seed_offset| {
            html! {
                <li class="list-item mb-5">
                    <PostCard seed={start_seed + seed_offset} />
                </li>
            }
        });
        html! {
            <div class="columns">
                <div class="column">
                    <ul class="list">
                        { for cards.by_ref().take(ITEMS_PER_PAGE as usize / 2) }
                    </ul>
                </div>
                <div class="column">
                    <ul class="list">
                        { for cards }
                    </ul>
                </div>
            </div>
        }
    };

    html! {
        <div class="section container">
            <h1 class="title">{ "Posts" }</h1>
            <h2 class="subtitle">{ "All of our quality writing in one place" }</h2>
            { posts }
            <Pagination
                page={current_page}
                total_pages={TOTAL_PAGES}
                route_to_page={Route::Posts}
            />
        </div>
    }
}
