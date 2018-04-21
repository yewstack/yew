
use yew::prelude::*;
use Context;
use yew::services::route::RouteInfo;
use yew::services::route::RouteSection;
use yew::services::route::Router;

use button::Button;

use Model;
use Msg;
use Route as MainRoute;

// Oftentimes the route doesn't need to hold any state or react to any changes, so it doesn't need to be a component.
#[derive(Clone, Debug, PartialEq)]
pub enum Route {
    CatForum,
    DogForum,
    ForumsList
}

// It can be seen that this can be
impl Router for Route {
    fn from_route(route: &mut RouteInfo) -> Option<Self> {
        if let Some(RouteSection::Node{segment}) = route.next() {
            match segment.as_str() {
                "cat" => Some(Route::CatForum),
                "dog" => Some(Route::DogForum),
                _ => Some(Route::ForumsList) // If the route can't be resolved, return None to let the parent router know that it should redirect to a failed route.
            }
        } else {
            Some(Route::ForumsList)
        }
    }
    fn to_route(&self) -> RouteInfo {
        match *self {
            Route::CatForum => RouteInfo::parse("/cat").unwrap(), // TODO I would like to refactor this into a macro that will fail at compile time if the parse fails
            Route::DogForum => RouteInfo::parse("/dog").unwrap(),
            Route::ForumsList => RouteInfo::parse("/").unwrap()
        }
    }
}

// Renderable needs to have the generic signature of the parent component, in this case, Model.
impl Renderable<Context, Model> for Route {
    fn view(&self) -> Html<Context, Model> {
        match *self {
            Route::CatForum => {
                html! {
                    // Conceptually, these could also be components to which routing props can be passed
                    <>
                        {"I'm the forum for talking about cats"}
                    </>
                }
            }
            Route::DogForum => {
                html! {
                    <>
                        {"I'm the forum for talking about dogs"}
                    </>
                }
            }
            Route::ForumsList => {
                html!{
                    <div>
                        <div>
                            <Button: title="Dog forum", onsignal=|_| Msg::Navigate(MainRoute::Forums(Route::DogForum)) ,/>
                        </div>
                        <div>
                            <Button: title="Cat forum", onsignal=|_| Msg::Navigate(MainRoute::Forums(Route::CatForum)) ,/>
                        </div>
                    </div>
                }
            }
        }
    }
}
