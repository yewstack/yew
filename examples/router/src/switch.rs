use yew_router::{components::RouterAnchor, router::Router, switch::Permissive, Switch};

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    #[to = "/posts/{}"]
    Post(u64),
    #[to = "/posts/?page={}"]
    PostListPage(u64),
    #[to = "/posts/"]
    PostList,
    #[to = "/authors/{id}"]
    Author(u64),
    #[to = "/authors"]
    AuthorList,
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
    #[to = "/!"]
    Home,
}

// type aliases to make life just a bit easier

pub type AppRouter = Router<AppRoute>;
pub type AppAnchor = RouterAnchor<AppRoute>;
