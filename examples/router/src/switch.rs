use yew::{
    virtual_dom::{Transformer, VComp},
    web_sys::Url,
};
use yew_router::{components::RouterAnchor, prelude::*, switch::Permissive};

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    #[at = "/posts/{}"]
    Post(u64),
    #[at = "/posts/?page={}"]
    PostListPage(u64),
    #[at = "/posts/"]
    PostList,
    #[at = "/authors/{id}"]
    Author(u64),
    #[at = "/authors/"]
    AuthorList,
    #[at = "/page-not-found"]
    PageNotFound(Permissive<String>),
    #[at = "/!"]
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
///
/// IMPORTANT: You *must* specify a `<base>` tag on your webpage in order for this to work!
/// For more information, see the 
/// [Mozilla Developer Network docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/base)
#[derive(Clone, Debug)]
pub struct PublicUrlSwitch(AppRoute);

impl PublicUrlSwitch {
    fn base_url() -> Url {
        if let Ok(Some(href)) = yew::utils::document().base_uri() {
            // since this always returns an absolute URL we turn it into `Url`
            // so we can more easily get the path.
            Url::new(&href).unwrap()
        } else {
            Url::new("/").unwrap()
        }
    }

    fn base_path() -> String {
        let mut path = Self::base_url().pathname();
        if path.ends_with('/') {
            // pop the trailing slash because AppRoute already accounts for it
            path.pop();
        }

        path
    }

    pub fn route(self) -> AppRoute {
        self.0
    }
}
impl Switch for PublicUrlSwitch {
    fn from_route_part<STATE>(part: String, state: Option<STATE>) -> (Option<Self>, Option<STATE>) {
        if let Some(part) = part.strip_prefix(&Self::base_path()) {
            let (route, state) = AppRoute::from_route_part(part.to_owned(), state);
            (route.map(Self), state)
        } else {
            (None, None)
        }
    }

    fn build_route_section<STATE>(self, route: &mut String) -> Option<STATE> {
        route.push_str(&Self::base_path());
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
