use std::borrow::Cow;

use super::parsing::{extract_named_args, insert_named_args, remove_segment};
use super::Args;

/// This trait is implemented for each possible way arguments can be
/// extracted from a route.
pub trait RouteProcessor {
    fn apply(&self, args: &mut Args) -> bool;
    fn unapply(&self, args: &mut Args);
}

// Configuration for the `PathSegmentMuncher`
#[derive(Debug)]
pub enum PathPart {
    Match(&'static str),
    ExtractOne,
    ExtractAll,
}

/// A muncher which matches and/or extracts path segments of a URL.
pub struct PathSegmentProcessor {
    pub allow_partial: bool,
    pub parts: &'static [PathPart],
}

impl RouteProcessor for PathSegmentProcessor {
    fn apply(&self, args: &mut Args) -> bool {
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

        if !self.allow_partial && route.path != "/" {
            // We should be matching the entire route, but there was
            // part of the path left over, so consider this not a match.
            return false;
        }

        args.store_route(route);

        true
    }

    fn unapply(&self, args: &mut Args) {
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

/// A muncher which extracts query arguments from a URL by name.
pub struct QueryArgProcessor {
    pub names: &'static [&'static str],
}

impl RouteProcessor for QueryArgProcessor {
    fn apply(&self, args: &mut Args) -> bool {
        let mut route = args.take_route();
        extract_named_args("?", self.names, &mut route.query, args);
        args.store_route(route);
        true
    }

    fn unapply(&self, args: &mut Args) {
        let mut route = args.take_route();
        insert_named_args("?", self.names, &mut route.query, args);
        args.store_route(route);
    }
}

/// A muncher which extracts URL-encoded hash arguments by name.
pub struct HashArgProcessor {
    pub names: &'static [&'static str],
}

impl RouteProcessor for HashArgProcessor {
    fn apply(&self, args: &mut Args) -> bool {
        let mut route = args.take_route();
        extract_named_args("#", self.names, &mut route.hash, args);
        args.store_route(route);
        true
    }

    fn unapply(&self, args: &mut Args) {
        let mut route = args.take_route();
        insert_named_args("#", self.names, &mut route.hash, args);
        args.store_route(route);
    }
}
