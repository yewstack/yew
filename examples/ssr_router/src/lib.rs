use std::collections::HashMap;

use function_router::content;
#[cfg(not(target_arch = "wasm32"))]
use function_router::Generated;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_link::{linked_state, use_linked_state, LinkProvider, ResolverProp};
use yew_router::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkedPost(pub content::Post);

#[linked_state]
impl LinkedState for LinkedPost {
    type Context = ();
    type Input = u32;

    async fn resolve(_ctx: &(), seed: &u32) -> Self {
        Self(content::Post::generate_from_seed(*seed))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkedAuthor(pub content::Author);

#[linked_state]
impl LinkedState for LinkedAuthor {
    type Context = ();
    type Input = u32;

    async fn resolve(_ctx: &(), seed: &u32) -> Self {
        Self(content::Author::generate_from_seed(*seed))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkedPostMeta(pub content::PostMeta);

#[linked_state]
impl LinkedState for LinkedPostMeta {
    type Context = ();
    type Input = u32;

    async fn resolve(_ctx: &(), seed: &u32) -> Self {
        Self(content::PostMeta::generate_from_seed(*seed))
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct PostProps {
    pub id: u32,
}

#[component]
pub fn PostPage(props: &PostProps) -> HtmlResult {
    let post = use_linked_state::<LinkedPost>(props.id)?.unwrap();
    Ok(render_post(&post.0))
}

fn render_post(post: &content::Post) -> Html {
    use content::PostPart;

    let render_quote = |quote: &content::Quote| {
        html! {
            <article class="media block box my-6">
                <figure class="media-left">
                    <p class="image is-64x64">
                        <img alt="The author's profile" src={quote.author.image_url.clone()} loading="lazy" />
                    </p>
                </figure>
                <div class="media-content">
                    <div class="content">
                        <Link<function_router::Route> classes={classes!("is-size-5")} to={function_router::Route::Author { id: quote.author.seed }}>
                            <strong>{ &quote.author.name }</strong>
                        </Link<function_router::Route>>
                        <p class="is-family-secondary">{ &quote.content }</p>
                    </div>
                </div>
            </article>
        }
    };

    let render_section_hero = |section: &content::Section| {
        html! {
            <section class="hero is-dark has-background mt-6 mb-3">
                <img alt="Section image" class="hero-background is-transparent" src={section.image_url.clone()} loading="lazy" />
                <div class="hero-body">
                    <div class="container">
                        <h2 class="subtitle">{ &section.title }</h2>
                    </div>
                </div>
            </section>
        }
    };

    let render_section = |section: &content::Section, show_hero: bool| {
        let hero = if show_hero {
            render_section_hero(section)
        } else {
            html! {}
        };
        html! {
            <section>
                { hero }
                <div>
                    for p in section.paragraphs.iter() {
                        <p>{ p }</p>
                    }
                </div>
            </section>
        }
    };

    let view_content = {
        let mut show_hero = false;
        let parts: Vec<Html> = post
            .content
            .iter()
            .map(|part| match part {
                PostPart::Section(section) => {
                    let html = render_section(section, show_hero);
                    show_hero = true;
                    html
                }
                PostPart::Quote(quote) => {
                    show_hero = false;
                    render_quote(quote)
                }
            })
            .collect();
        html! {
            {for parts}
        }
    };

    html! {
        <>
            <section class="hero is-medium is-light has-background">
                <img alt="Hero background" class="hero-background is-transparent" src={post.meta.image_url.clone()} />
                <div class="hero-body">
                    <div class="container">
                        <h1 class="title">{ &post.meta.title }</h1>
                        <h2 class="subtitle">
                            { "by " }
                            <Link<function_router::Route> classes={classes!("has-text-weight-semibold")} to={function_router::Route::Author { id: post.meta.author.seed }}>
                                { &post.meta.author.name }
                            </Link<function_router::Route>>
                        </h2>
                        <div class="tags">
                            for kw in &post.meta.keywords {
                                <span class="tag is-info">{ kw }</span>
                            }
                        </div>
                    </div>
                </div>
            </section>
            <div class="section container">{ view_content }</div>
        </>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct AuthorProps {
    pub id: u32,
}

#[component]
pub fn AuthorPage(props: &AuthorProps) -> HtmlResult {
    let author = use_linked_state::<LinkedAuthor>(props.id)?.unwrap();
    Ok(render_author(&author.0))
}

fn render_author(author: &content::Author) -> Html {
    html! {
        <div class="section container">
            <div class="tile is-ancestor is-vertical">
                <div class="tile is-parent">
                    <article class="tile is-child notification is-light">
                        <p class="title">{ &author.name }</p>
                    </article>
                </div>
                <div class="tile">
                    <div class="tile is-parent is-3">
                        <article class="tile is-child notification">
                            <p class="title">{ "Interests" }</p>
                            <div class="tags">
                                for tag in &author.keywords {
                                    <span class="tag is-info">{ tag }</span>
                                }
                            </div>
                        </article>
                    </div>
                    <div class="tile is-parent">
                        <figure class="tile is-child image is-square">
                            <img alt="Profile picture" src={author.image_url.clone()} />
                        </figure>
                    </div>
                    <div class="tile is-parent">
                        <article class="tile is-child notification is-info">
                            <div class="content">
                                <p class="title">{ "About me" }</p>
                                <div class="content">
                                    { "This author has chosen not to reveal anything about themselves" }
                                </div>
                            </div>
                        </article>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct CardProps {
    seed: u32,
}

#[component]
fn LinkedPostCard(props: &CardProps) -> HtmlResult {
    let meta = use_linked_state::<LinkedPostMeta>(props.seed)?.unwrap();
    let meta = &meta.0;
    Ok(html! {
        <div class="card">
            <div class="card-image">
                <figure class="image is-2by1">
                    <img alt="This post's image" src={meta.image_url.clone()} loading="lazy" />
                </figure>
            </div>
            <div class="card-content">
                <Link<function_router::Route> classes={classes!("title", "is-block")} to={function_router::Route::Post { id: meta.seed }}>
                    { &meta.title }
                </Link<function_router::Route>>
                <Link<function_router::Route> classes={classes!("subtitle", "is-block")} to={function_router::Route::Author { id: meta.author.seed }}>
                    { &meta.author.name }
                </Link<function_router::Route>>
            </div>
        </div>
    })
}

#[component]
fn LinkedAuthorCard(props: &CardProps) -> HtmlResult {
    let author = use_linked_state::<LinkedAuthor>(props.seed)?.unwrap();
    let author = &author.0;
    Ok(html! {
        <div class="card">
            <div class="card-content">
                <div class="media">
                    <div class="media-left">
                        <figure class="image is-128x128">
                            <img alt="Author's profile picture" src={author.image_url.clone()} />
                        </figure>
                    </div>
                    <div class="media-content">
                        <p class="title is-3">{ &author.name }</p>
                        <p>
                            { "I like " }
                            <b>{ author.keywords.join(", ") }</b>
                        </p>
                    </div>
                </div>
            </div>
            <footer class="card-footer">
                <Link<function_router::Route> classes={classes!("card-footer-item")} to={function_router::Route::Author { id: author.seed }}>
                    { "Profile" }
                </Link<function_router::Route>>
            </footer>
        </div>
    })
}

const ITEMS_PER_PAGE: u32 = 10;
const TOTAL_PAGES: u32 = u32::MAX / ITEMS_PER_PAGE;

#[component]
fn LinkedPostList() -> Html {
    use function_router::components::pagination::{PageQuery, Pagination};

    let location = use_location().unwrap();
    let current_page = location.query::<PageQuery>().map(|it| it.page).unwrap_or(1);

    let start_seed = (current_page - 1) * ITEMS_PER_PAGE;
    let half = ITEMS_PER_PAGE / 2;

    html! {
        <div class="section container">
            <h1 class="title">{ "Posts" }</h1>
            <h2 class="subtitle">{ "All of our quality writing in one place" }</h2>
            <div class="columns">
                <div class="column">
                    <ul class="list">
                        for offset in 0..half {
                            <li class="list-item mb-5">
                                <Suspense fallback={html! { <div class="card"><div class="card-content">{"Loading..."}</div></div> }}>
                                    <LinkedPostCard seed={start_seed + offset} />
                                </Suspense>
                            </li>
                        }
                    </ul>
                </div>
                <div class="column">
                    <ul class="list">
                        for offset in half..ITEMS_PER_PAGE {
                            <li class="list-item mb-5">
                                <Suspense fallback={html! { <div class="card"><div class="card-content">{"Loading..."}</div></div> }}>
                                    <LinkedPostCard seed={start_seed + offset} />
                                </Suspense>
                            </li>
                        }
                    </ul>
                </div>
            </div>
            <Pagination
                page={current_page}
                total_pages={TOTAL_PAGES}
                route_to_page={function_router::Route::Posts}
            />
        </div>
    }
}

#[component]
fn LinkedAuthorList() -> Html {
    let seeds = use_state(|| {
        use rand::{distr, Rng};
        rand::rng()
            .sample_iter(distr::StandardUniform)
            .take(2)
            .collect::<Vec<u32>>()
    });

    let on_complete = {
        let seeds = seeds.clone();
        Callback::from(move |_| {
            use rand::{distr, Rng};
            seeds.set(
                rand::rng()
                    .sample_iter(distr::StandardUniform)
                    .take(2)
                    .collect(),
            );
        })
    };

    html! {
        <div class="container">
            <section class="hero">
                <div class="hero-body">
                    <div class="container">
                        <h1 class="title">{ "Authors" }</h1>
                        <h2 class="subtitle">
                            { "Meet the definitely real people behind your favourite Yew content" }
                        </h2>
                    </div>
                </div>
            </section>
            <p class="section py-0">
                { "It wouldn't be fair " }
                <i>{ "(or possible :P)" }</i>
                {" to list each and every author in alphabetical order."}
                <br />
                { "So instead we chose to put more focus on the individuals by introducing you to two people at a time" }
            </p>
            <div class="section">
                <div class="tile is-ancestor">
                    for seed in seeds.iter().copied() {
                        <div class="tile is-parent">
                            <div class="tile is-child">
                                <Suspense fallback={html! { <div class="card"><div class="card-content">{"Loading..."}</div></div> }}>
                                    <LinkedAuthorCard {seed} />
                                </Suspense>
                            </div>
                        </div>
                    }
                </div>
                <function_router::components::progress_delay::ProgressDelay duration_ms={15000} on_complete={on_complete} />
            </div>
        </div>
    }
}

fn switch(routes: function_router::Route) -> Html {
    use function_router::Route;

    match routes {
        Route::Post { id } => html! {
            <Suspense fallback={html! { <p class="section container">{"Loading post..."}</p> }}>
                <PostPage {id} />
            </Suspense>
        },
        Route::Author { id } => html! {
            <Suspense fallback={html! { <p class="section container">{"Loading author..."}</p> }}>
                <AuthorPage {id} />
            </Suspense>
        },
        Route::Posts => html! { <LinkedPostList /> },
        Route::Authors => html! { <LinkedAuthorList /> },
        Route::Home => html! { <function_router::pages::home::Home /> },
        Route::NotFound => html! { <function_router::pages::page_not_found::PageNotFound /> },
    }
}

#[component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <LinkProvider endpoint="/api/link">
                <function_router::components::nav::Nav />
                <main>
                    <Switch<function_router::Route> render={switch} />
                </main>
                <footer class="footer">
                    <div class="content has-text-centered">
                        { "Powered by " }
                        <a href="https://yew.rs">{ "Yew" }</a>
                        { " using " }
                        <a href="https://bulma.io">{ "Bulma" }</a>
                    </div>
                </footer>
            </LinkProvider>
        </BrowserRouter>
    }
}

#[derive(Properties, PartialEq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
    pub queries: HashMap<String, String>,
    pub resolver: ResolverProp,
}

#[component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
    use yew_router::history::{AnyHistory, History, MemoryHistory};

    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

    html! {
        <Router history={history}>
            <LinkProvider endpoint="/api/link" resolver={props.resolver.clone()}>
                <function_router::components::nav::Nav />
                <main>
                    <Switch<function_router::Route> render={switch} />
                </main>
                <footer class="footer">
                    <div class="content has-text-centered">
                        { "Powered by " }
                        <a href="https://yew.rs">{ "Yew" }</a>
                        { " using " }
                        <a href="https://bulma.io">{ "Bulma" }</a>
                    </div>
                </footer>
            </LinkProvider>
        </Router>
    }
}
