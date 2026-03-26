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

yew_site_lib::spa_version_resolver!(
    resolve_next,
    "Next",
    sidebar,
    yew_site_docs_zh_hans,
    yew_site_docs_zh_hans::pages::getting_started::build_a_sample_app::page_content()
);
yew_site_lib::spa_version_resolver!(
    resolve_stable,
    "0.23",
    sidebar_0_23,
    yew_site_docs_zh_hans,
    yew_site_docs_zh_hans::pages::getting_started::build_a_sample_app::page_content_versioned(
        Some("0.23")
    )
);
yew_site_lib::spa_version_resolver!(
    resolve_v022,
    "0.22",
    sidebar_0_22,
    yew_site_docs_zh_hans,
    yew_site_docs_zh_hans::pages::getting_started::build_a_sample_app::page_content_versioned(
        Some("0.22")
    )
);
yew_site_lib::spa_version_resolver!(
    resolve_v021,
    "0.21",
    sidebar_0_21,
    yew_site_docs_zh_hans_0_21,
    yew_site_docs_zh_hans_0_21::pages::getting_started::build_a_sample_app::page_content()
);
yew_site_lib::spa_version_resolver!(
    resolve_v020,
    "0.20",
    sidebar_0_20,
    yew_site_docs_zh_hans_0_20,
    yew_site_docs_zh_hans_0_20::pages::getting_started::build_a_sample_app::page_content()
);

yew_site_lib::page_map!(resolve_migration, "", sidebar, [
    ("yew/from-0-19-0-to-0-20-0", "From 0.19.0 to 0.20.0", "/docs/migration-guides/yew/from-0-19-0-to-0-20-0", yew_site_docs::pages::migration_guides::yew::from_0_19_0_to_0_20_0::page_content()),
    ("yew/from-0-20-0-to-0-21-0", "From 0.20.0 to 0.21.0", "/docs/migration-guides/yew/from-0-20-0-to-0-21-0", yew_site_docs::pages::migration_guides::yew::from_0_20_0_to_0_21_0::page_content()),
    ("yew/from-0-21-0-to-0-22-0", "From 0.21.0 to 0.22.0", "/docs/migration-guides/yew/from-0-21-0-to-0-22-0", yew_site_docs::pages::migration_guides::yew::from_0_21_0_to_0_22_0::page_content()),
    ("yew/from-0-22-0-to-0-23-0", "From 0.22.0 to 0.23.0", "/docs/migration-guides/yew/from-0-22-0-to-0-23-0", yew_site_docs_zh_hans::pages::migration_guides::yew::from_0_22_0_to_0_23_0::page_content()),
    ("yew-agent/from-0-0-0-to-0-1-0", "From 0.0.0 to 0.1.0", "/docs/migration-guides/yew-agent/from-0-0-0-to-0-1-0", yew_site_docs::pages::migration_guides::yew_agent::from_0_0_0_to_0_1_0::page_content()),
    ("yew-agent/from-0-1-0-to-0-2-0", "From 0.1.0 to 0.2.0", "/docs/migration-guides/yew-agent/from-0-1-0-to-0-2-0", yew_site_docs::pages::migration_guides::yew_agent::from_0_1_0_to_0_2_0::page_content()),
    ("yew-agent/from-0-3-0-to-0-4-0", "From 0.3.0 to 0.4.0", "/docs/migration-guides/yew-agent/from-0-3-0-to-0-4-0", yew_site_docs::pages::migration_guides::yew_agent::from_0_3_0_to_0_4_0::page_content()),
    ("yew-agent/from-0-4-0-to-0-5-0", "From 0.4.0 to 0.5.0", "/docs/migration-guides/yew-agent/from-0-4-0-to-0-5-0", yew_site_docs_zh_hans::pages::migration_guides::yew_agent::from_0_4_0_to_0_5_0::page_content()),
    ("yew-router/from-0-15-0-to-0-16-0", "From 0.15.0 to 0.16.0", "/docs/migration-guides/yew-router/from-0-15-0-to-0-16-0", yew_site_docs_zh_hans::pages::migration_guides::yew_router::from_0_15_0_to_0_16_0::page_content()),
    ("yew-router/from-0-16-0-to-0-17-0", "From 0.16.0 to 0.17.0", "/docs/migration-guides/yew-router/from-0-16-0-to-0-17-0", yew_site_docs::pages::migration_guides::yew_router::from_0_16_0_to_0_17_0::page_content()),
    ("yew-router/from-0-19-0-to-0-20-0", "From 0.19.0 to 0.20.0", "/docs/migration-guides/yew-router/from-0-19-0-to-0-20-0", yew_site_docs_zh_hans::pages::migration_guides::yew_router::from_0_19_0_to_0_20_0::page_content()),
]);

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
