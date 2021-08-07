use crate::components::{pagination::Pagination, post_card::PostCard};
use crate::Route;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

const ITEMS_PER_PAGE: u64 = 10;
const TOTAL_PAGES: u64 = u64::MAX / ITEMS_PER_PAGE;

pub enum Msg {
    ShowPage(u64),
}

#[derive(Serialize, Deserialize)]
struct PageQuery {
    page: u64,
}

pub struct PostList {
    link: ComponentLink<Self>,
}
impl Component for PostList {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ShowPage(page) => {
                yew_router::push_route_with_query(Route::Posts, PageQuery { page }).unwrap();
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let page = self.current_page();

        html! {
            <div class="section container">
                <h1 class="title">{ "Posts" }</h1>
                <h2 class="subtitle">{ "All of our quality writing in one place" }</h2>
                { self.view_posts() }
                <Pagination
                    page={page}
                    total_pages={TOTAL_PAGES}
                    on_switch_page={self.link.callback(Msg::ShowPage)}
                />
            </div>
        }
    }
}
impl PostList {
    fn view_posts(&self) -> Html {
        let start_seed = (self.current_page() - 1) * ITEMS_PER_PAGE;
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
    }

    fn current_page(&self) -> u64 {
        yew_router::parse_query::<PageQuery>()
            .map(|it| it.page)
            .unwrap_or(1)
    }
}
