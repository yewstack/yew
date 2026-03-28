use yew::prelude::*;
#[cfg(feature = "csr")]
use yew_router::prelude::*;
#[cfg(feature = "csr")]
use yew_site_lib::{Layout, NavigationContext};

#[cfg(feature = "csr")]
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/next/")]
    HomeNext,
    #[at("/0.22/")]
    HomeV022,
    #[at("/0.21/")]
    HomeV021,
    #[at("/0.20/")]
    HomeV020,
    #[at("/tutorial/")]
    Tutorial,
    #[at("/next/tutorial/")]
    TutorialNext,
    #[at("/0.22/tutorial/")]
    TutorialV022,
    #[at("/0.21/tutorial/")]
    TutorialV021,
    #[at("/0.20/tutorial/")]
    TutorialV020,
    #[at("/docs/next/*path")]
    DocsNext { path: String },
    #[at("/docs/0.22/*path")]
    DocsV022 { path: String },
    #[at("/docs/0.21/*path")]
    DocsV021 { path: String },
    #[at("/docs/0.20/*path")]
    DocsV020 { path: String },
    #[at("/docs/migration-guides/*path")]
    MigrationGuides { path: String },
    #[at("/docs/*path")]
    DocsStable { path: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

yew_site_lib::spa_version_resolver!(
    resolve_next,
    "Next",
    yew_site_docs::sidebar_data::docs_sidebar,
    yew_site_docs
);
yew_site_lib::spa_version_resolver!(
    resolve_stable,
    "0.23",
    yew_site_docs_0_23::sidebar_data::docs_sidebar,
    yew_site_docs_0_23
);
yew_site_lib::spa_version_resolver!(
    resolve_v022,
    "0.22",
    yew_site_docs_0_22::sidebar_data::docs_sidebar,
    yew_site_docs_0_22
);

yew_site_lib::spa_version_resolver!(
    resolve_v021,
    "0.21",
    yew_site_docs_0_21::sidebar_data::docs_sidebar,
    yew_site_docs_0_21
);

yew_site_lib::spa_version_resolver!(
    resolve_v020,
    "0.20",
    yew_site_docs_0_20::sidebar_data::docs_sidebar,
    yew_site_docs_0_20
);

yew_site_lib::spa_migration_resolver!(
    resolve_migration,
    yew_site_docs::sidebar_data::docs_sidebar,
    yew_site_docs
);

yew_site_lib::spa_csr_boilerplate!(
    "",
    "en",
    "",
    yew_site_docs,
    yew_site_docs_0_21,
    yew_site_docs_0_20
);

yew_site_lib::spa_ssr_render_pages!(
    "", "",
    yew_site_docs, yew_site_docs_0_21, yew_site_docs_0_20,
    tutorial_sidebar_next: vec![],
    tutorial_sidebar_stable: vec![],
    tutorial_sidebar_0_22: vec![],
    tutorial_sidebar_0_21: vec![],
    tutorial_sidebar_0_20: vec![],
    home_pages: [yew_site_home::Page, yew_site_home::PageNext, yew_site_home::PageV022, yew_site_home::PageV021, yew_site_home::PageV020]
);
