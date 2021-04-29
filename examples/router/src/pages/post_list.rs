use crate::components::{pagination::Pagination, post_card::PostCard};
use crate::Routes;
use std::collections::HashMap;
use yew::prelude::*;
use yew_router::RouterService;

const ITEMS_PER_PAGE: u64 = 10;
const TOTAL_PAGES: u64 = std::u64::MAX / ITEMS_PER_PAGE;

pub enum Msg {
    ShowPage(u64),
}

pub struct PostList {
    link: ComponentLink<Self>,
}
impl Component for PostList {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ShowPage(page) => {
                RouterService::push(Routes::Posts, {
                    let mut map = HashMap::new();
                    map.insert("page", page.to_string());
                    Some(map)
                });
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
                    page=page
                    total_pages=TOTAL_PAGES
                    on_switch_page=self.link.callback(Msg::ShowPage)
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
                    <PostCard seed=start_seed + seed_offset />
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
        RouterService::query()
            .get("page")
            .map(|it| it.parse().expect("invalid page"))
            .unwrap_or(1)
    }
}
