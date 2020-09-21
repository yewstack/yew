use crate::{
    components::{pagination::Pagination, post_card::PostCard},
    switch::AppRoute,
};
use yew::prelude::*;
use yew_router::agent::{RouteAgentDispatcher, RouteRequest};
use yewtil::NeqAssign;

const ITEMS_PER_PAGE: u64 = 10;
const TOTAL_PAGES: u64 = std::u64::MAX / ITEMS_PER_PAGE;

pub enum Msg {
    ShowPage(u64),
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub page: u64,
}

pub struct PostList {
    props: Props,
    link: ComponentLink<Self>,
    route_dispatcher: RouteAgentDispatcher,
}
impl Component for PostList {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            route_dispatcher: RouteAgentDispatcher::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ShowPage(page) => {
                let route = AppRoute::PostListPage(page);
                self.route_dispatcher
                    .send(RouteRequest::ChangeRoute(route.into_route()));
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let Props { page } = self.props;

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
        let start_seed = (self.props.page - 1) * ITEMS_PER_PAGE;
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
