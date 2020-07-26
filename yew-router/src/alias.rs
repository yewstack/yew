/// Generates a module named `router_state` containing aliases to common structures within
/// yew_router that deal with operating with Route and its state values as well as functions for
/// rendering routes.
///
/// Because they should be the same across a given application,
/// its a handy way to make sure that every type that could be needed is generated.
///
/// This macro is used to generate aliases for the state type of `()` within yew_router.
/// Instead of doing these yourself, use this macro if you need to store state in the browser.
///
/// # Example
/// ```
/// # use yew_router::define_router_state;
/// define_router_state!(Option<String>);
/// use router_state::Route; // alias to Route<Option<String>>
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! define_router_state {
    ($StateT:ty) => {
        define_router_state!($StateT, stringify!($StateT));
    };
    ($StateT:ty, $StateName:expr) => {
        #[doc = "A set of aliases to commonly used structures and functions used for routing."]
        mod router_state {

            #[doc = "The state that can be stored by the router service."]
            pub type State = $StateT;

            #[doc = "Alias to [Route<"]
            #[doc = $StateName]
            #[doc = ">](route/struct.Route.html)."]
            pub type Route = $crate::route::Route<$StateT>;

            #[doc = "Alias to [RouteService<"]
            #[doc = $StateName]
            #[doc = ">](route_service/struct.RouteService.html)."]
            pub type RouteService = $crate::service::RouteService<$StateT>;

            #[cfg(feature="agent")]
            #[doc = "Alias to [RouteAgent<"]
            #[doc = $StateName]
            #[doc = ">](agent/struct.RouteAgent.html)."]
            pub type RouteAgent = $crate::agent::RouteAgent<$StateT>;

            #[cfg(feature="agent")]
            #[doc = "Alias to [RouteAgentBridge<"]
            #[doc = $StateName]
            #[doc = ">](agent/bridge/struct.RouteAgentBridge.html)`."]
            pub type RouteAgentBridge = $crate::agent::RouteAgentBridge<$StateT>;

            #[cfg(feature="agent")]
            #[doc = "Alias to [RouteAgentDispatcher<"]
            #[doc = $StateName]
            #[doc = ">](agent/struct.RouteAgentDispatcher.html)`."]
            pub type RouteAgentDispatcher = $crate::agent::RouteAgentDispatcher<$StateT>;


            #[allow(deprecated)]
            #[deprecated(note = "Has been renamed to RouterAnchor")]
            #[cfg(feature="components")]
            #[doc = "Alias to [RouterLink<"]
            #[doc = $StateName]
            #[doc = ">](components/struct.RouterLink.html)`."]
            pub type RouterLink = $crate::components::RouterLink<$StateT>;


            #[cfg(feature="components")]
            #[doc = "Alias to [RouterAnchor<"]
            #[doc = $StateName]
            #[doc = ">](components/struct.RouterAnchor.html)`."]
            pub type RouterAnchor = $crate::components::RouterAnchor<$StateT>;

            #[cfg(feature="components")]
            #[doc = "Alias to [RouterButton<"]
            #[doc = $StateName]
            #[doc = ">](components/struct.RouterButton.html)`."]
            pub type RouterButton = $crate::components::RouterButton<$StateT>;

            #[cfg(feature="router")]
            #[doc = "Alias to [Router<"]
            #[doc = $StateName]
            #[doc = ">](router/router/struct.Router.html)."]
            pub type Router<SW> = $crate::router::Router<$StateT, SW>;

        }
    }
}
