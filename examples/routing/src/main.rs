
#[macro_use]
extern crate yew;

mod forums;
mod button;

use yew::prelude::*;
use yew::html::Scope;
use yew::services::route::*;

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

impl From<RouteResult> for Msg {
    fn from( result: RouteResult) -> Self {
        match result {
            Ok(route_info) => {
               Msg::Navigate(Route::from(&route_info))
            }
            Err(e) => {
                eprintln!("Couldn't route: {:?}", e);
                Msg::Navigate(Route::PageNotFoundRoute)
            }
        }
    }

}

impl <'a> From<&'a RouteInfo> for Route {
    fn from(route_info: &RouteInfo) -> Self {
        println!("Converting from url");
        if let Some(first_segment) = route_info.get_segment_at_index(0) {
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
        match self {
            // You can add RouteInfos together to combine paths in logical order.
            // The fragment and query of the rhs take precedence over any fragment or query set by the lhs.
            Route::Forums(forum_route)=> RouteInfo::parse("/forums").unwrap() + forum_route.into(),
            Route::PageNotFoundRoute => RouteInfo::parse("/PageNotFound").unwrap(),
        }
    }
}



impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, context: &mut Env<Context, Self>) -> Self {

        // TODO This is sort of a hack around rust's borrow checker rules.
        // If anything better can be proposed, I would appreciate it.
        let callback = RouteService::create_routing_callback::<Self, Context>(context);
        context.routing.register_router(callback);


        let route: Route = (&context.routing.get_current_route_info()).into();
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
        let page = |route: &Route| {
            match *route {
                Route::Forums(ref forum_route) => {
                    html!{
                        <>
                            // The beauty of this is that the Forums component isn't recreated when
                            // the route changes, it only calls the Forums.change() method.
                            //
                            // So if the Forums component holds onto some data from a network
                            // request or user input, that data isn't affected by the component's
                            // route prop changing,
                            <Forums: route=forum_route, />
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