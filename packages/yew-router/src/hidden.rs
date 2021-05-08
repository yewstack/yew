use std::any::{Any, TypeId};
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::fmt::Display;
use std::mem;
use std::str::FromStr;

use percent_encoding::{percent_decode_str, percent_encode, NON_ALPHANUMERIC};
use regex::RegexSet;
use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen::JsValue;

use crate::history::Route;
pub use crate::Routable;

/// This trait is implemented by the derive macro. There is a blanket implementation of
/// `Routeable` for all `DerivedRoutable`.
pub trait DerivedRoutable: PartialEq + Any + Default + Clone {
    const VARIANTS: &'static [RouteMuncher<Self>];

    fn to_args(&self) -> (usize, Args);
}

/// Lazily builds and caches a `RegexSet` keyed on the TypeId of the `DerivedRoutable`.
fn cached_regex_set<T: DerivedRoutable>() -> &'static RegexSet {
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
            if let Some(res) = variant.from_route(route) {
                return Ok(res);
            }
        }
        Err(T::default())
    }

    fn to_route(&self) -> Route {
        let (index, args) = self.to_args();
        let variant = &T::VARIANTS[index];
        variant.reconstitute(args)
    }
}

#[derive(Debug, Clone)]
pub enum Arg {
    Empty,
    String(String),
    Opaque(JsValue),
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

impl From<JsValue> for Arg {
    fn from(v: JsValue) -> Self {
        if v.is_undefined() {
            Arg::Empty
        } else {
            Arg::Opaque(v)
        }
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

impl TryFrom<Arg> for JsValue {
    type Error = ();

    fn try_from(value: Arg) -> Result<Self, Self::Error> {
        if let Arg::Opaque(v) = value {
            Ok(v)
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
    pub fn push_serde<T: Serialize>(&mut self, arg: &T) {
        self.push(JsValue::from_serde(arg).expect("Argument to serialize"));
    }
    pub fn pop_serde<T: DeserializeOwned>(&mut self) -> Option<T> {
        self.pop::<JsValue>()?.into_serde().ok()
    }
    pub fn push_str<T: Display>(&mut self, arg: &T) {
        self.push(arg.to_string());
    }
    pub fn pop_str<T: FromStr>(&mut self) -> Option<T> {
        self.pop::<String>()?.parse().ok()
    }
    pub fn take_route(&mut self) -> Route {
        mem::replace(&mut self.route, Default::default())
    }
    pub fn store_route(&mut self, route: Route) {
        self.route = route;
    }
}

/// An instance of this type is constructed for each variant by the
/// derive macro.
pub struct RouteMuncher<T: DerivedRoutable> {
    pub regex: &'static str,
    pub munchers: &'static [&'static dyn Muncher],
    pub ctor: &'static dyn Fn(Args) -> Option<T>,
}

impl<T: DerivedRoutable> RouteMuncher<T> {
    /// Extracts a list of arguments from the `Route` using the configured
    /// munchers.
    fn munch(&self, route: &Route) -> Option<Args> {
        let mut args = Args::new(route.clone());
        for muncher in self.munchers {
            if !muncher.munch(&mut args) {
                return None;
            }
        }
        Some(args)
    }
    /// Attempts to construct this variant from the provided `Route`.
    fn from_route(&self, route: &Route) -> Option<T> {
        (self.ctor)(self.munch(route)?)
    }
    /// Builds a route from a list of arguments.
    fn reconstitute(&self, mut args: Args) -> Route {
        for muncher in self.munchers.iter().rev() {
            muncher.reconstitute(&mut args);
        }
        args.take_route()
    }
}

/// This trait is implemented for each possible way arguments can be
/// extracted from a route.
pub trait Muncher {
    fn munch(&self, args: &mut Args) -> bool;
    fn reconstitute(&self, args: &mut Args);
}

// Configuration for the `PathSegmentMuncher`
pub enum PathPart {
    Match(&'static str),
    ExtractOne,
    ExtractAll,
}

/// A muncher which matches and/or extracts path segments of a URL.
pub struct PathSegmentMuncher {
    pub parts: &'static [PathPart],
}

impl Muncher for PathSegmentMuncher {
    fn munch(&self, args: &mut Args) -> bool {
        fn remove_segment<'a>(path: &mut &'a str) -> Option<&'a str> {
            *path = &path[1..];
            let i = path.find('/').unwrap_or(path.len());
            if i > 0 {
                let res = &path[0..i];
                if i < path.len() {
                    *path = &path[i..];
                } else {
                    *path = "/"
                }
                Some(res)
            } else {
                None
            }
        }

        let mut route = args.take_route();
        let mut path = route.path.as_str();

        if path.is_empty() {
            return false;
        }

        for part in self.parts {
            match part {
                PathPart::Match(literal) => {
                    if remove_segment(&mut path) != Some(literal) {
                        return false;
                    }
                }
                PathPart::ExtractOne => {
                    if let Some(segment) = remove_segment(&mut path) {
                        args.push(segment);
                    } else {
                        return false;
                    }
                }
                PathPart::ExtractAll => {
                    args.push(path);
                    path = "";
                }
            }
        }
        route.path = if path.is_empty() {
            "/".into()
        } else {
            path.into()
        };
        args.store_route(route);

        true
    }

    fn reconstitute(&self, args: &mut Args) {
        let mut route = args.take_route();
        let mut rev_path_acc = vec![if route.path == "/" {
            Cow::Borrowed("")
        } else {
            Cow::Owned(route.path)
        }];

        for part in self.parts.iter().rev() {
            rev_path_acc.push(match *part {
                PathPart::Match(literal) => literal.into(),
                PathPart::ExtractOne => Cow::Owned(args.pop().expect("Missing path parameter")),
                PathPart::ExtractAll => Cow::Owned(args.pop().expect("Missing path parameter")),
            });
            rev_path_acc.push(Cow::Borrowed("/"));
        }

        rev_path_acc.reverse();
        route.path = rev_path_acc.concat();

        if route.path.is_empty() {
            route.path = "/".into();
        }
        args.store_route(route);
    }
}

fn munch_named_args(prefix: &str, names: &[&str], value: &mut String, args: &mut Args) {
    if names.is_empty() {
        return;
    }
    let mut arr = vec![Arg::Empty; names.len()];
    if let Some(remainder) = value.strip_prefix(prefix) {
        let mut unused_parts = Vec::new();
        for part in remainder.split('&') {
            let mut it = part.splitn(2, '=');
            if let (Some(lhs), Some(rhs)) = (it.next(), it.next()) {
                if let Some(i) = names.iter().position(|&name| name == lhs) {
                    let rhs = rhs.replace('+', " ");
                    let rhs = percent_decode_str(rhs.as_str()).decode_utf8_lossy();
                    arr[i] = rhs.as_ref().into();
                    continue;
                }
            }
            unused_parts.push(part);
        }
        if unused_parts.is_empty() {
            value.clear();
        } else {
            *value = format!("{}{}", prefix, unused_parts.join("&"));
        }
    }
    for item in arr {
        args.push(item);
    }
}

fn reconstitute_named_args(prefix: &str, names: &[&str], value: &mut String, args: &mut Args) {
    if names.is_empty() {
        return;
    }
    let existing = value.strip_prefix(prefix).unwrap_or(&value);

    let mut rev_arg_acc = if existing.is_empty() {
        vec![]
    } else {
        vec![Cow::Borrowed(existing)]
    };
    for &name in names.iter().rev() {
        if let Some(value) = args.pop::<String>() {
            if !rev_arg_acc.is_empty() {
                rev_arg_acc.push("&".into());
            }

            let rhs: Cow<str> = percent_encode(value.as_bytes(), NON_ALPHANUMERIC).into();
            rev_arg_acc.push(rhs.replace("%20", "+").into());
            rev_arg_acc.push("=".into());
            rev_arg_acc.push(name.into());
        }
    }
    if !rev_arg_acc.is_empty() {
        rev_arg_acc.push(prefix.into());
    }
    rev_arg_acc.reverse();
    *value = rev_arg_acc.concat();
}

/// A muncher which extracts query arguments from a URL by name.
pub struct QueryArgMuncher {
    pub names: &'static [&'static str],
}

impl Muncher for QueryArgMuncher {
    fn munch(&self, args: &mut Args) -> bool {
        let mut route = args.take_route();
        munch_named_args("?", self.names, &mut route.query, args);
        args.store_route(route);
        true
    }

    fn reconstitute(&self, args: &mut Args) {
        let mut route = args.take_route();
        reconstitute_named_args("?", self.names, &mut route.query, args);
        args.store_route(route);
    }
}

/// A muncher which extracts URL-encoded hash arguments by name.
pub struct HashArgMuncher {
    pub names: &'static [&'static str],
}

impl Muncher for HashArgMuncher {
    fn munch(&self, args: &mut Args) -> bool {
        let mut route = args.take_route();
        munch_named_args("#", self.names, &mut route.query, args);
        args.store_route(route);
        true
    }

    fn reconstitute(&self, args: &mut Args) {
        let mut route = args.take_route();
        reconstitute_named_args("#", self.names, &mut route.query, args);
        args.store_route(route);
    }
}
