use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::nav::Nav;
use crate::pages::{
    author::Author, author_list::AuthorList, home::Home, page_not_found::PageNotFound, post::Post,
    post_list::PostList,
};

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/posts/:id")]
    Post { id: u32 },
    #[at("/posts")]
    Posts,
    #[at("/authors/:id")]
    Author { id: u32 },
    #[at("/authors")]
    Authors,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Nav />

            <main>
                <Switch<Route> render={Switch::render(switch)} />
            </main>
            <footer class="footer">
                <div class="content has-text-centered">
                    { "Powered by " }
                    <a href="https://yew.rs">{ "Yew" }</a>
                    { " using " }
                    <a href="https://bulma.io">{ "Bulma" }</a>
                    { " and images from " }
                    <a href="https://unsplash.com">{ "Unsplash" }</a>
                </div>
            </footer>
        </BrowserRouter>
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod arch_native {
    use super::*;

    use yew::virtual_dom::AttrValue;
    use yew_router::history::{AnyHistory, History, MemoryHistory};

    use std::collections::HashMap;

    #[derive(Properties, PartialEq, Debug)]
    pub struct ServerAppProps {
        pub url: AttrValue,
        pub queries: HashMap<String, String>,
    }

    #[function_component]
    pub fn ServerApp(props: &ServerAppProps) -> Html {
        let history = AnyHistory::from(MemoryHistory::new());
        history
            .push_with_query(&*props.url, &props.queries)
            .unwrap();

        html! {
            <Router history={history}>
                <Nav />

                <main>
                    <Switch<Route> render={Switch::render(switch)} />
                </main>
                <footer class="footer">
                    <div class="content has-text-centered">
                        { "Powered by " }
                        <a href="https://yew.rs">{ "Yew" }</a>
                        { " using " }
                        <a href="https://bulma.io">{ "Bulma" }</a>
                        { " and images from " }
                        <a href="https://unsplash.com">{ "Unsplash" }</a>
                    </div>
                </footer>
            </Router>
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use arch_native::*;

fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::Post { id } => {
            html! { <Post seed={id} /> }
        }
        Route::Posts => {
            html! { <PostList /> }
        }
        Route::Author { id } => {
            html! { <Author seed={id} /> }
        }
        Route::Authors => {
            html! { <AuthorList /> }
        }
        Route::Home => {
            html! { <Home /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}
