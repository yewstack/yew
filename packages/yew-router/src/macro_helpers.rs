use std::any::Any;
use std::convert::{TryFrom, TryInto};
use std::fmt::Display;
use std::mem;
use std::str::FromStr;

#[cfg(feature = "regex")]
use regex::RegexSet;

use crate::history::Route;
pub use crate::Routable;

mod parsing;
mod processors;

pub use processors::*;

/// This trait is implemented by the derive macro. There is a blanket implementation of
/// `Routeable` for all `DerivedRoutable`.
pub trait DerivedRoutable: PartialEq + Any + Default + Clone {
    const VARIANTS: &'static [RouteVariant<Self>];

    fn to_variant_index_and_args(&self) -> (usize, Args);
}

/// Lazily builds and caches a `RegexSet` keyed on the TypeId of the `DerivedRoutable`.
#[cfg(feature = "regex")]
fn cached_regex_set<T: DerivedRoutable>() -> &'static RegexSet {
    use std::any::TypeId;
    use std::cell::RefCell;
    use std::collections::HashMap;

    thread_local! {
        static REGEX_CACHE: RefCell<HashMap<TypeId, &'static RegexSet>> = RefCell::new(HashMap::new());
    }

    // In the fast path, we simply lookup the TypeId in the map.
    REGEX_CACHE.with(|cache| {
        *cache
            .borrow_mut()
            .entry(TypeId::of::<T>())
            .or_insert_with(|| {
                // In the slow path we build a `RegexSet` from all the individual variant regexes. Leaking
                // this `RegexSet` is fine, because the total number of distinct `DerivedRoutable` types is
                // fixed at compile time.
                Box::leak(Box::new(
                    RegexSet::new(T::VARIANTS.iter().map(|v| v.regex))
                        .expect("Invalid routing regex"),
                ))
            })
    })
}

/// Blanket implementation of `Routable` for `DerivedRoutable`.
impl<T: DerivedRoutable> Routable for T {
    #[cfg(feature = "regex")]
    fn from_route(route: &Route) -> Result<Self, Self>
    where
        Self: Sized,
    {
        // Use a `RegexSet` to rapidly narrow the search to a small number of
        // candidate routes.
        let regex_set = cached_regex_set::<T>();
        for candidate in regex_set.matches(&route.path) {
            // Try each route in turn. `RegexSet` guarantees iteration order to
            // match the original array.
            let variant = &T::VARIANTS[candidate];
            if let Some(res) = variant.try_parse(route) {
                return Ok(res);
            }
        }
        Err(T::default())
    }

    #[cfg(not(feature = "regex"))]
    fn from_route(route: &Route) -> Result<Self, Self>
    where
        Self: Sized,
    {
        // Try all variants in turn
        for variant in T::VARIANTS {
            if let Some(res) = variant.try_parse(route) {
                return Ok(res);
            }
        }
        Err(T::default())
    }

    fn to_route(&self) -> Route {
        let (index, args) = self.to_variant_index_and_args();
        let variant = &T::VARIANTS[index];
        variant.build_route(args)
    }
}

#[derive(Debug, Clone)]
pub enum Arg {
    Empty,
    String(String),
}

impl From<String> for Arg {
    fn from(s: String) -> Self {
        Arg::String(s)
    }
}

impl From<&str> for Arg {
    fn from(s: &str) -> Self {
        Arg::String(s.into())
    }
}

impl TryFrom<Arg> for String {
    type Error = ();

    fn try_from(value: Arg) -> Result<Self, Self::Error> {
        if let Arg::String(s) = value {
            Ok(s)
        } else {
            Err(())
        }
    }
}

#[derive(Debug)]
pub struct Args {
    args: Vec<Arg>,
    route: Route,
}

impl Args {
    pub fn new(route: Route) -> Self {
        Self {
            route,
            args: Vec::new(),
        }
    }
    pub fn empty() -> Self {
        Self::new(Route::new("/"))
    }
    pub fn push<T: Into<Arg>>(&mut self, arg: T) {
        self.args.push(arg.into());
    }
    pub fn pop<T>(&mut self) -> Option<T>
    where
        Arg: TryInto<T>,
    {
        self.args.pop().expect("No argument to pop").try_into().ok()
    }
    pub fn push_str<T: Display>(&mut self, arg: &T) {
        self.push(arg.to_string());
    }
    pub fn pop_str<T: FromStr>(&mut self) -> Option<T> {
        self.pop::<String>()?.parse().ok()
    }
    pub fn take_route(&mut self) -> Route {
        mem::replace(&mut self.route, Route::empty())
    }
    pub fn store_route(&mut self, route: Route) {
        self.route = route;
    }
}
/// An instance of this type is constructed for each variant by the
/// derive macro.
pub struct RouteVariant<T: DerivedRoutable> {
    pub regex: &'static str,
    pub processors: &'static [&'static dyn RouteProcessor],
    pub ctor: &'static dyn Fn(Args) -> Option<T>,
}

impl<T: DerivedRoutable> RouteVariant<T> {
    /// Attempts to construct this variant from the provided `Route`.
    fn try_parse(&self, route: &Route) -> Option<T> {
        let mut args = Args::new(route.clone());
        for processor in self.processors {
            if !processor.apply(&mut args) {
                return None;
            }
        }
        (self.ctor)(args)
    }
    /// Builds a route from a list of arguments.
    fn build_route(&self, mut args: Args) -> Route {
        for processor in self.processors.iter().rev() {
            processor.unapply(&mut args);
        }
        args.take_route()
    }
}
