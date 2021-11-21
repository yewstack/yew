use yew::prelude::*;

const ELLIPSIS: &str = "\u{02026}";

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub page: u64,
    pub total_pages: u64,
    pub on_switch_page: Callback<u64>,
}

pub struct Pagination;

impl Component for Pagination {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <nav class="pagination is-right" role="navigation" aria-label="pagination">
                { self.view_relnav_buttons(ctx.props()) }
                <ul class="pagination-list">
                    { self.view_links(ctx.props()) }
                </ul>
            </nav>
        }
    }
}
impl Pagination {
    fn render_link(&self, to_page: u64, props: &Props) -> Html {
        let Props {
            page,
            ref on_switch_page,
            ..
        } = *props;

        let onclick = on_switch_page.reform(move |_| to_page);
        let is_current_class = if to_page == page { "is-current" } else { "" };

        html! {
            <li>
                <a class={classes!("pagination-link", is_current_class)} aria-label={format!("Goto page {}", to_page)} {onclick}>
                    { to_page }
                </a>
            </li>
        }
    }

    fn render_links<P>(&self, mut pages: P, len: usize, max_links: usize, props: &Props) -> Html
    where
        P: Iterator<Item = u64> + DoubleEndedIterator,
    {
        if len > max_links {
            let last_link = self.render_link(pages.next_back().unwrap(), props);
            // remove 1 for the ellipsis and 1 for the last link
            let links = pages
                .take(max_links - 2)
                .map(|page| self.render_link(page, props));
            html! {
                <>
                    { for links }
                    <li><span class="pagination-ellipsis">{ ELLIPSIS }</span></li>
                    { last_link }
                </>
            }
        } else {
            html! { for pages.map(|page| self.render_link(page, props)) }
        }
    }

    fn view_links(&self, props: &Props) -> Html {
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
                { self.render_links(1..page, pages_prev, links_left, props) }
                <li>{ self.render_link(page, props) }</li>
                { self.render_links(page + 1..=total_pages, pages_next, links_right, props) }
            </>
        }
    }

    fn view_relnav_buttons(&self, props: &Props) -> Html {
        let Props {
            page,
            total_pages,
            ref on_switch_page,
        } = *props;

        html! {
            <>
                <a class="pagination-previous"
                    disabled={page==1}
                    onclick={on_switch_page.reform(move |_| page - 1)}
                >
                    { "Previous" }
                </a>
                <a class="pagination-next"
                    disabled={page==total_pages}
                    onclick={on_switch_page.reform(move |_| page + 1)}
                >
                    { "Next page" }
                </a>
            </>
        }
    }
}
