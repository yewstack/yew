pub mod components;
mod current_route;
mod params;
pub mod router;
mod service;
pub(crate) mod utils;

pub use current_route::CurrentRoute;
pub use params::Params;
pub use service::RouterService;

pub mod prelude {
    pub use crate::components::{link::Link, route::Route};
    pub use crate::router::Router;
    #[doc(no_inline)]
    pub use crate::{CurrentRoute, Params, RouterService};
}
