use yew_router::{route::Route, switch::Permissive, Switch};

fn main() {
    let route = Route::new_no_state("/some/route");
    let app_route = AppRoute::switch(route);
    dbg!(app_route);

    let route = Route::new_no_state("/some/thing/other");
    let app_route = AppRoute::switch(route);
    dbg!(app_route);

    let route = Route::new_no_state("/another/other");
    let app_route = AppRoute::switch(route);
    dbg!(app_route);

    let route = Route::new_no_state("/inner/left");
    let app_route = AppRoute::switch(route);
    dbg!(app_route);

    let route = Route::new_no_state("/yeet"); // should not match
    let app_route = AppRoute::switch(route);
    dbg!(app_route);

    let route = Route::new_no_state("/single/32");
    let app_route = AppRoute::switch(route);
    dbg!(app_route);

    let route = Route::new_no_state("/othersingle/472");
    let app_route = AppRoute::switch(route);
    dbg!(app_route);

    let route = Route::new_no_state("/option/test");
    let app_route = AppRoute::switch(route);
    dbg!(app_route);

    let mut buf = String::new();
    AppRoute::Another("yeet".to_string()).build_route_section::<()>(&mut buf);
    println!("{}", buf);

    let mut buf = String::new();
    AppRoute::Something {
        thing: "yeet".to_string(),
        other: "yote".to_string(),
    }
    .build_route_section::<()>(&mut buf);
    println!("{}", buf);

    let mut buf = String::new();
    OtherSingle(23).build_route_section::<()>(&mut buf);
    println!("{}", buf);
}

#[derive(Debug, Switch, Clone)]
pub enum AppRoute {
    #[to = "/some/route"]
    SomeRoute,
    #[to = "/some/{thing}/{other}"]
    // If you have a variant with named fields, the field names should appear in the matcher string.
    Something { thing: String, other: String },
    #[to = "/another/{}"] // Tuple-enums don't need names in the capture groups.
    Another(String),
    #[to = "/doot/{}/{something}"]
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
    #[to = "/option/{}"]
    Optional(Permissive<String>),
    /// Because this is permissive, a corresponding capture group doesn't need to exist
    #[to = "/missing/capture"]
    MissingCapture(Permissive<String>),
}

#[derive(Switch, Debug, Clone)]
pub enum InnerRoute {
    #[to = "/left"]
    Left,
    #[to = "/right"]
    Right,
}

#[derive(Switch, Debug, Clone)]
#[to = "/single/{number}"]
pub struct Single {
    number: u32,
}

#[derive(Switch, Debug, Clone)]
#[to = "/othersingle/{number}"]
pub struct OtherSingle(u32);

//#[derive(Switch, Debug)]
// pub enum Bad {
//    #[to = "/bad_route/{hello}"]
//    X,
//}

#[derive(Switch, Debug, Clone)]
#[to = "{*:path}#{route}"]
pub struct FragmentAdapter<W: Switch> {
    path: String,
    route: W,
}
