use std::ops::Range;

use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

const ELLIPSIS: &str = "\u{02026}";

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct PageQuery {
    pub page: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub page: u32,
    pub total_pages: u32,
    pub route_to_page: Route,
}

#[function_component]
pub fn RelNavButtons(props: &Props) -> Html {
    let Props {
        page,
        total_pages,
        route_to_page: to,
    } = props.clone();

    html! {
        <>
            <Link<Route, PageQuery>
                classes={classes!("pagination-previous")}
                disabled={page==1}
                query={Some(PageQuery{page: page - 1})}
                to={to.clone()}
            >
                { "Previous" }
            </Link<Route, PageQuery>>
            <Link<Route, PageQuery>
                classes={classes!("pagination-next")}
                disabled={page==total_pages}
                query={Some(PageQuery{page: page + 1})}
                {to}
            >
                { "Next page" }
            </Link<Route, PageQuery>>
        </>
    }
}

#[derive(Properties, Clone, Debug, PartialEq, Eq)]
pub struct RenderLinksProps {
    range: Range<u32>,
    len: usize,
    max_links: usize,
    props: Props,
}

#[function_component]
pub fn RenderLinks(props: &RenderLinksProps) -> Html {
    let RenderLinksProps {
        range,
        len,
        max_links,
        props,
    } = props.clone();

    let mut range = range;

    if len > max_links {
        let last_link =
            html! {<RenderLink to_page={range.next_back().unwrap()} props={props.clone()} />};
        // remove 1 for the ellipsis and 1 for the last link
        let links = range
            .take(max_links - 2)
            .map(|page| html! {<RenderLink to_page={page} props={props.clone()} />});
        html! {
            <>
                { for links }
                <li><span class="pagination-ellipsis">{ ELLIPSIS }</span></li>
                { last_link }
            </>
        }
    } else {
        html! { for range.map(|page| html! {<RenderLink to_page={page} props={props.clone()} />}) }
    }
}

#[derive(Properties, Clone, Debug, PartialEq, Eq)]
pub struct RenderLinkProps {
    to_page: u32,
    props: Props,
}

#[function_component]
pub fn RenderLink(props: &RenderLinkProps) -> Html {
    let RenderLinkProps { to_page, props } = props.clone();

    let Props {
        page,
        route_to_page,
        ..
    } = props;

    let is_current_class = if to_page == page { "is-current" } else { "" };

    html! {
        <li>
            <Link<Route, PageQuery>
                classes={classes!("pagination-link", is_current_class)}
                to={route_to_page}
                query={Some(PageQuery{page: to_page})}
            >
                { to_page }
            </Link<Route, PageQuery>>
        </li>
    }
}

#[function_component]
pub fn Links(props: &Props) -> Html {
    const LINKS_PER_SIDE: usize = 3;

    let Props {
        page, total_pages, ..
    } = *props;

    let pages_prev = page.checked_sub(1).unwrap_or_default() as usize;
    let pages_next = (total_pages - page) as usize;

    let links_left = LINKS_PER_SIDE.min(pages_prev)
            // if there are less than `LINKS_PER_SIDE` to the right, we add some more on the left.
            + LINKS_PER_SIDE.checked_sub(pages_next).unwrap_or_default();
    let links_right = 2 * LINKS_PER_SIDE - links_left;

    html! {
        <>
            <RenderLinks range={ 1..page } len={pages_prev} max_links={links_left} props={props.clone()} />
            <RenderLink to_page={page} props={props.clone()} />
            <RenderLinks range={ page + 1..total_pages + 1 } len={pages_next} max_links={links_right} props={props.clone()} />
        </>
    }
}

#[function_component]
pub fn Pagination(props: &Props) -> Html {
    html! {
        <nav class="pagination is-right" role="navigation" aria-label="pagination">
            <RelNavButtons ..{props.clone()} />
            <ul class="pagination-list">
                <Links ..{props.clone()} />
            </ul>
        </nav>
    }
}
