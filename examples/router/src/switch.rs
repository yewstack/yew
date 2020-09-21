use default_env::default_env;
use yew::virtual_dom::{Transformer, VComp};
use yew_router::{components::RouterAnchor, prelude::*, switch::Permissive};

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
    #[to = "/authors/"]
    AuthorList,
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
    #[to = "/!"]
    Home,
}
impl AppRoute {
    pub fn into_public(self) -> PublicUrlSwitch {
        PublicUrlSwitch(self)
    }

    pub fn into_route(self) -> Route {
        Route::from(self.into_public())
    }
}

/// Helper type which just wraps around the actual `AppRoute` but handles a public url prefix.
/// We need to have this because we're hosting the example at `/router/` instead of `/`.
/// This type allows us have the best of both worlds.
#[derive(Clone, Debug)]
pub struct PublicUrlSwitch(AppRoute);
impl PublicUrlSwitch {
    // this variable is set by the build script
    const PUBLIC_URL: &'static str = default_env!("PUBLIC_URL", "/");

    /// Return `PUBLIC_URL` without a trailing slash.
    /// This is required because `AppRoute` still expects paths to
    /// start with a slash so we only want to strip away the parts before that.
    fn public_url_no_trailing_slash() -> &'static str {
        Self::PUBLIC_URL
            .strip_suffix('/')
            .unwrap_or(Self::PUBLIC_URL)
    }

    pub fn route(self) -> AppRoute {
        self.0
    }
}
impl Switch for PublicUrlSwitch {
    fn from_route_part<STATE>(part: String, state: Option<STATE>) -> (Option<Self>, Option<STATE>) {
        if let Some(part) = part.strip_prefix(Self::public_url_no_trailing_slash()) {
            let (route, state) = AppRoute::from_route_part(part.to_owned(), state);
            (route.map(Self), state)
        } else {
            (None, None)
        }
    }

    fn build_route_section<STATE>(self, route: &mut String) -> Option<STATE> {
        route.push_str(Self::public_url_no_trailing_slash());
        self.0.build_route_section(route)
    }
}

// this allows us to pass `AppRoute` to components which take `PublicUrlSwitch`.

impl Transformer<AppRoute, PublicUrlSwitch> for VComp {
    fn transform(from: AppRoute) -> PublicUrlSwitch {
        from.into_public()
    }
}

// type aliases to make life just a bit easier

pub type AppRouter = Router<PublicUrlSwitch>;
pub type AppAnchor = RouterAnchor<PublicUrlSwitch>;
