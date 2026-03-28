use yew::prelude::*;
#[cfg(feature = "csr")]
use yew_router::prelude::*;
#[cfg(feature = "csr")]
use yew_site_lib::{Layout, NavigationContext};

#[cfg(feature = "csr")]
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/zh-Hans/")]
    Home,
    #[at("/zh-Hans/next/")]
    HomeNext,
    #[at("/zh-Hans/0.22/")]
    HomeV022,
    #[at("/zh-Hans/0.21/")]
    HomeV021,
    #[at("/zh-Hans/0.20/")]
    HomeV020,
    #[at("/zh-Hans/tutorial/")]
    Tutorial,
    #[at("/zh-Hans/next/tutorial/")]
    TutorialNext,
    #[at("/zh-Hans/0.22/tutorial/")]
    TutorialV022,
    #[at("/zh-Hans/0.21/tutorial/")]
    TutorialV021,
    #[at("/zh-Hans/0.20/tutorial/")]
    TutorialV020,
    #[at("/zh-Hans/docs/next/*path")]
    DocsNext { path: String },
    #[at("/zh-Hans/docs/0.22/*path")]
    DocsV022 { path: String },
    #[at("/zh-Hans/docs/0.21/*path")]
    DocsV021 { path: String },
    #[at("/zh-Hans/docs/0.20/*path")]
    DocsV020 { path: String },
    #[at("/zh-Hans/docs/migration-guides/*path")]
    MigrationGuides { path: String },
    #[at("/zh-Hans/docs/*path")]
    DocsStable { path: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

yew_site_lib::spa_sidebar_fns!(
    "zh-Hans",
    yew_site_docs_zh_hans,
    yew_site_docs_zh_hans_0_23,
    yew_site_docs_zh_hans_0_22,
    yew_site_docs_zh_hans_0_21,
    yew_site_docs_zh_hans_0_20
);

yew_site_lib::spa_version_resolver!(resolve_next, "Next", sidebar, yew_site_docs_zh_hans);
yew_site_lib::spa_version_resolver!(
    resolve_stable,
    "0.23",
    sidebar_0_23,
    yew_site_docs_zh_hans_0_23
);
yew_site_lib::spa_version_resolver!(
    resolve_v022,
    "0.22",
    sidebar_0_22,
    yew_site_docs_zh_hans_0_22
);
yew_site_lib::spa_version_resolver!(
    resolve_v021,
    "0.21",
    sidebar_0_21,
    yew_site_docs_zh_hans_0_21
);
yew_site_lib::spa_version_resolver!(
    resolve_v020,
    "0.20",
    sidebar_0_20,
    yew_site_docs_zh_hans_0_20
);

yew_site_lib::spa_migration_resolver!(resolve_migration, sidebar, yew_site_docs_zh_hans);

yew_site_lib::spa_csr_boilerplate!(
    "zh-Hans",
    "zh-Hans",
    "/zh-Hans",
    yew_site_docs_zh_hans,
    yew_site_docs_zh_hans_0_21,
    yew_site_docs_zh_hans_0_20
);

yew_site_lib::spa_ssr_render_pages!(
    "zh-Hans", "/zh-Hans",
    yew_site_docs_zh_hans, yew_site_docs_zh_hans_0_21, yew_site_docs_zh_hans_0_20,
    tutorial_sidebar_next: vec![],
    tutorial_sidebar_stable: vec![],
    tutorial_sidebar_0_22: vec![],
    tutorial_sidebar_0_21: vec![],
    tutorial_sidebar_0_20: vec![],
    home_pages: [yew_site_home::PageZhHans, yew_site_home::PageZhHansNext, yew_site_home::PageZhHansV022, yew_site_home::PageZhHansV021, yew_site_home::PageZhHansV020]
);
