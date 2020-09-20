use yew_router::{
    components::RouterAnchor,
    router::Router,
    switch::{AllowMissing, Permissive},
    Switch,
};

#[derive(Clone, Debug, Switch)]
pub enum PostsRoute {
    #[to = "/{num}"]
    Id(u64),
    #[to = "/"]
    List,
}

#[derive(Clone, Debug, Switch)]
#[to = "/{id}"]
pub struct AuthorRoute(pub u64);

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    #[to = "/posts{*:inner}"]
    Posts(PostsRoute),
    #[to = "/authors{*:inner}"]
    Authors(AllowMissing<AuthorRoute>),
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
    #[to = "/!"]
    Home,
}

// type aliases to make life just a bit easier

pub type AppRouter = Router<AppRoute>;
pub type AppAnchor = RouterAnchor<AppRoute>;
