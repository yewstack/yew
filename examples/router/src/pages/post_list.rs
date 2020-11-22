use crate::{
    components::{pagination::Pagination, post_card::PostCard},
    switch::AppRoute,
};
use yew::prelude::*;
use yew_router::agent::{RouteAgentDispatcher, RouteRequest};

const ITEMS_PER_PAGE: u64 = 10;
const TOTAL_PAGES: u64 = std::u64::MAX / ITEMS_PER_PAGE;

pub enum Msg {
    ShowPage(u64),
}

#[derive(Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub page: u64,
}

pub struct PostList {
    route_dispatcher: RouteAgentDispatcher,
}

impl Component for PostList {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            route_dispatcher: RouteAgentDispatcher::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ShowPage(page) => {
                let route = AppRoute::PostListPage(page);
                self.route_dispatcher
                    .send(RouteRequest::ChangeRoute(route.into_route()));
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="section container">
                <h1 class="title">{ "Posts" }</h1>
                <h2 class="subtitle">{ "All of our quality writing in one place" }</h2>
                { self.view_posts(ctx) }
                <Pagination
                    page=ctx.props.page
                    total_pages=TOTAL_PAGES
                    on_switch_page=ctx.callback(Msg::ShowPage)
                />
            </div>
        }
    }
}
impl PostList {
    fn view_posts(&self, ctx: &Context<Self>) -> Html {
        let start_seed = (ctx.props.page - 1) * ITEMS_PER_PAGE;
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
}
