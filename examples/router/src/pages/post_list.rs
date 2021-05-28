use crate::components::{pagination::Pagination, post_card::PostCard};
use crate::Route;
use yew::prelude::*;
use yew_router::{Router, RouterAction};

const ITEMS_PER_PAGE: u64 = 10;
const TOTAL_PAGES: u64 = u64::MAX / ITEMS_PER_PAGE;

#[derive(Properties, Clone)]
pub struct PostListProps {
    pub page: u64,
}

pub struct PostList {
    router: Router<crate::Route>,
    props: PostListProps,
}
impl Component for PostList {
    type Message = ();
    type Properties = PostListProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router = Router::new(link);
        Self { router, props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let on_switch_page = self
            .router
            .dispatcher(|page| Some(RouterAction::Push(Route::Posts { page })));

        html! {
            <div class="section container">
                <h1 class="title">{ "Posts" }</h1>
                <h2 class="subtitle">{ "All of our quality writing in one place" }</h2>
                { self.view_posts() }
                <Pagination
                    page=self.props.page
                    total_pages=TOTAL_PAGES
                    on_switch_page=on_switch_page
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
