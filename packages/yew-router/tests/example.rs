use yew_router::{route::Route, switch::Permissive, Switch};

#[derive(Clone, Debug, Eq, PartialEq, Switch)]
pub enum InnerRoute {
    #[at = "/left"]
    Left,
    #[to = "/right"]
    Right,
}

#[derive(Clone, Debug, Eq, PartialEq, Switch)]
#[at = "/single/{number}"]
pub struct Single {
    number: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Switch)]
#[at = "/othersingle/{number}"]
pub struct OtherSingle(u32);
#[derive(Clone, Debug, Eq, PartialEq, Switch)]
#[at = "{*:path}#{route}"]
pub struct FragmentAdapter<W: Switch> {
    path: String,
    route: W,
}

#[derive(Clone, Debug, PartialEq, Switch)]
pub enum AppRoute {
    #[at = "/some/route"]
    SomeRoute,
    #[to = "/some/{thing}/{other}"]
    // If you have a variant with named fields, the field names should appear in the matcher string.
    Something { thing: String, other: String },
    #[at = "/another/{}"] // Tuple-enums don't need names in the capture groups.
    Another(String),
    #[at = "/doot/{}/{something}"]
    // You can still puts names in the capture groups to improve readability.
    Yeet(String, String),
    #[to = "/inner"]
    #[rest] // same as /inner{*}
    Nested(InnerRoute),
    #[rest] // Rest delegates the remaining input to the next attribute
    Single(Single),
    #[rest]
    OtherSingle(OtherSingle),
    /// Because this is permissive, the inner item doesn't have to match.
    #[at = "/option/{}"]
    Optional(Permissive<String>),
    /// Because this is permissive, a corresponding capture group doesn't need to exist
    #[at = "/missing/capture"]
    MissingCapture(Permissive<String>),
}

#[test]
fn switch() {
    let route = Route::new_no_state("/some/route");
    assert_eq!(AppRoute::switch(route), Some(AppRoute::SomeRoute));

    let route = Route::new_no_state("/some/thing/other");
    assert_eq!(
        AppRoute::switch(route),
        Some(AppRoute::Something {
            thing: "thing".to_owned(),
            other: "other".to_owned()
        })
    );

    let route = Route::new_no_state("/another/other");
    assert_eq!(
        AppRoute::switch(route),
        Some(AppRoute::Another("other".to_owned()))
    );

    let route = Route::new_no_state("/inner/left");
    assert_eq!(
        AppRoute::switch(route),
        Some(AppRoute::Nested(InnerRoute::Left))
    );

    let route = Route::new_no_state("/yeet");
    assert_eq!(AppRoute::switch(route), None);

    let route = Route::new_no_state("/single/32");
    assert_eq!(
        AppRoute::switch(route),
        Some(AppRoute::Single(Single { number: 32 }))
    );

    let route = Route::new_no_state("/othersingle/472");
    assert_eq!(
        AppRoute::switch(route),
        Some(AppRoute::OtherSingle(OtherSingle(472)))
    );

    let route = Route::new_no_state("/option/test");
    assert_eq!(
        AppRoute::switch(route),
        Some(AppRoute::Optional(Permissive(Some("test".to_owned()))))
    );
}

#[test]
fn build_route() {
    let mut buf = String::new();
    AppRoute::Another("yeet".to_string()).build_route_section::<()>(&mut buf);
    assert_eq!(buf, "/another/yeet");

    let mut buf = String::new();
    AppRoute::Something {
        thing: "yeet".to_string(),
        other: "yote".to_string(),
    }
    .build_route_section::<()>(&mut buf);
    assert_eq!(buf, "/some/yeet/yote");

    let mut buf = String::new();
    OtherSingle(23).build_route_section::<()>(&mut buf);
    assert_eq!(buf, "/othersingle/23");
}
