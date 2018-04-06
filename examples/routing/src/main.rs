
#[macro_use]
extern crate yew;

mod forums;
mod button;

use yew::prelude::*;
use yew::html::Scope;
use yew::services::route::*;
use yew::url::ParseError;

use yew::html::Renderable;

use forums::Forums;
use button::Button;



pub struct Context {
    routing: RouteService
}

struct Model {
    route: Route
}

#[derive(Clone, Debug)]
enum Route {
    Forums(forums::Route),
    PageNotFoundRoute
}


enum Msg {
    Navigate(Route),
}

impl From<ParseError> for Msg {
    fn from(error: ParseError) -> Msg {
        println!("Couldn't parse url: {:?}", error);
        Msg::Navigate(Route::PageNotFoundRoute)
    }
}

impl From<Route> for Msg {
    fn from(route: Route) -> Msg {
        Msg::Navigate(route)
    }
}

impl From<RouteInfo> for Route {
    fn from(route_info: RouteInfo) -> Self {
        println!("Converting from url");
        if let Some(first_segment) = route_info.clone().path_segments.get(0).map(String::as_str) {
            println!("matching: {}", first_segment);
            match first_segment {
                "forums" => return Route::Forums(forums::Route::from(route_info)),
                _ => return Route::PageNotFoundRoute
            }
        }
        Route::PageNotFoundRoute
    }
}

impl Into<RouteInfo> for Route {
    fn into(self) -> RouteInfo {
        match self.clone() {
            // But what about the case where this is nested 3+ layers deep?
            // The current approach won't scale for that.
            Route::Forums(forums_route) => RouteInfo::from(vec![self.into(), forums_route.into()]), // TODO this is far from perfect. I would like some feedback.
            Route::PageNotFoundRoute => RouteInfo::from(vec![self.into()])
        }
    }
}

impl Into<PathSegment> for Route {
    fn into(self) -> PathSegment {
        match self {
            // TODO, I would like a try_into().expect("") pattern here instead when possible.
            // If the route is something user-defined, like a slug for an article's url, it could be
            // possible for a '/' to appear.
            // This also could be done using a macro that checks for '/'s at compile time.
            Route::Forums(_)=> "forums".into(),
            Route::PageNotFoundRoute => "PageNotFound".into(),
        }
    }
}



impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, context: &mut Env<Context, Self>) -> Self {

        // TODO This is sort of a hack around rust's borrow checker rules.
        // If anything better can be proposed, I would appreciate it.
        let callback = RouteService::create_routing_callback::<Route, Self, Context>(context);
        context.routing.register_router::<Route, Self, Context>(callback);


        let route: Route = context.routing.get_route_info_from_current_path().into();
        // TODO I may need to set the route here, but I don't want to make set_route public
        // TODO Maybe a redirect method that erases the most recent state in the history api, and replaces it with a new one?
        // ^^ would this call the callback? Because I don't want that.
        Model {
            route
        }
    }

    fn update(&mut self, msg: Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Navigate(route) => {
                println!("Main route: Navigating");
                context.routing.call_link(route.clone());
                self.route = route;
                true
            }
        }
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        let page = |page: &Route| {
            match page {
                Route::Forums(route) => {
                    html!{
                        <>
                            // The beauty of this is that the Forums component isn't recreated when
                            // the route changes, it only calls the Forums.change() method.
                            //
                            // So if the Forums component has stored some data from a network
                            // request or user input that isn't affected by its route field changing,
                            // it won't be lost.
                            <Forums: route=route, />
                        </>
                    }
                }
                Route::PageNotFoundRoute => {
                    html! {
                        <>
                            {"Page not found"}
                        </>
                    }
                }
            }
        };
        html! {
            <div>
                {"This could be some html that will be on every page, like a header."}
                <Button: title="GoToForums", onsignal=|_| Msg::Navigate(Route::Forums(forums::Route::ForumsList) ) ,/>
                <div>
                    {page(&self.route)}
                </div>
            </div>
        }
    }
}


fn main() {
    yew::initialize();
    let context = Context {
        routing: RouteService::new()
    };
    // We use `Scope` here for demonstration.
    // You can also use `App` here too.
    let app: Scope<Context, Model> = Scope::new(context);
    app.mount_to_body();
    yew::run_loop();
}