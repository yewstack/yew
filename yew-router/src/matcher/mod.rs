//! Module for matching route strings based on tokens generated from the yew_router_route_parser
//! crate.

mod matcher_impl;
mod util;

use nom::IResult;
use std::collections::HashSet;
use yew_router_route_parser::{parse_str_and_optimize_tokens, PrettyParseError};

pub use yew_router_route_parser::{CaptureVariant, Captures, MatcherToken};

/// Attempts to match routes, transform the route to Component props and render that Component.
#[derive(Debug, PartialEq, Clone)]
pub struct RouteMatcher {
    /// Tokens used to determine how the matcher will match a route string.
    pub tokens: Vec<MatcherToken>,
    /// Settings
    pub settings: MatcherSettings,
}

/// Settings used for the matcher.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MatcherSettings {
    /// All literal matches do not care about case.
    pub case_insensitive: bool,
}

impl Default for MatcherSettings {
    fn default() -> Self {
        MatcherSettings {
            case_insensitive: false,
        }
    }
}

impl RouteMatcher {
    /// Attempt to create a RouteMatcher from a "matcher string".
    pub fn try_from(i: &str) -> Result<Self, PrettyParseError> {
        let settings = MatcherSettings::default();
        Self::new(i, settings)
    }

    /// Creates a new Matcher with settings.
    pub fn new(i: &str, settings: MatcherSettings) -> Result<Self, PrettyParseError> {
        Ok(RouteMatcher {
            tokens: parse_str_and_optimize_tokens(
                i,
                yew_router_route_parser::FieldNamingScheme::Unnamed, // The most permissive scheme
            )?, /* TODO this field type should be a superset of Named, but it would be better to source this from settings, and make sure that the macro generates settings as such. */
            settings,
        })
    }

    /// Match a route string, collecting the results into a map.
    pub fn capture_route_into_map<'a, 'b: 'a>(
        &'b self,
        i: &'a str,
    ) -> IResult<&'a str, Captures<'a>> {
        matcher_impl::match_into_map(&self.tokens, &self.settings)(i)
    }

    /// Match a route string, collecting the results into a vector.
    pub fn capture_route_into_vec<'a, 'b: 'a>(
        &'b self,
        i: &'a str,
    ) -> IResult<&'a str, Vec<String>> {
        matcher_impl::match_into_vec(&self.tokens, &self.settings)(i)
    }

    /// Gets a set of all names that will be captured.
    /// This is useful in determining if a given struct will be able to be populated by a given path
    /// matcher before being given a concrete path to match.
    pub fn capture_names(&self) -> HashSet<&str> {
        fn capture_names_impl(tokens: &[MatcherToken]) -> HashSet<&str> {
            tokens
                .iter()
                .fold(HashSet::new(), |mut acc: HashSet<&str>, token| {
                    match token {
                        MatcherToken::Exact(_) | MatcherToken::End => {}
                        MatcherToken::Capture(capture) => match &capture {
                            CaptureVariant::ManyNamed(name)
                            | CaptureVariant::Named(name)
                            | CaptureVariant::NumberedNamed { name, .. } => {
                                acc.insert(&name);
                            }
                            CaptureVariant::Unnamed
                            | CaptureVariant::ManyUnnamed
                            | CaptureVariant::NumberedUnnamed { .. } => {}
                        },
                    }
                    acc
                })
        }
        capture_names_impl(&self.tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yew_router_route_parser::{
        convert_tokens,
        parser::{RefCaptureVariant, RouteParserToken},
    };

    impl<'a> From<Vec<RouteParserToken<'a>>> for RouteMatcher {
        fn from(tokens: Vec<RouteParserToken<'a>>) -> Self {
            let settings = MatcherSettings::default();
            RouteMatcher {
                tokens: convert_tokens(&tokens),
                settings,
            }
        }
    }

    #[test]
    fn basic_separator() {
        let tokens = vec![RouteParserToken::Separator];
        let path_matcher = RouteMatcher::from(tokens);
        path_matcher
            .capture_route_into_map("/")
            .expect("should parse");
    }

    #[test]
    fn multiple_tokens() {
        let tokens = vec![
            RouteParserToken::Separator,
            RouteParserToken::Exact("lorem"),
            RouteParserToken::Separator,
        ];

        let path_matcher = RouteMatcher::from(tokens);
        path_matcher
            .capture_route_into_map("/lorem/")
            .expect("should parse");
    }

    #[test]
    fn simple_capture() {
        let tokens = vec![
            RouteParserToken::Separator,
            RouteParserToken::Capture(RefCaptureVariant::Named("lorem")),
            RouteParserToken::Separator,
        ];
        let path_matcher = RouteMatcher::from(tokens);
        let (_, matches) = path_matcher
            .capture_route_into_map("/ipsum/")
            .expect("should parse");
        assert_eq!(matches["lorem"], "ipsum".to_string())
    }

    #[test]
    fn simple_capture_with_no_trailing_separator() {
        let tokens = vec![
            RouteParserToken::Separator,
            RouteParserToken::Capture(RefCaptureVariant::Named("lorem")),
        ];
        let path_matcher = RouteMatcher::from(tokens);
        let (_, matches) = path_matcher
            .capture_route_into_map("/ipsum")
            .expect("should parse");
        assert_eq!(matches["lorem"], "ipsum".to_string())
    }

    #[test]
    fn match_with_trailing_match_many() {
        let tokens = vec![
            RouteParserToken::Separator,
            RouteParserToken::Exact("a"),
            RouteParserToken::Separator,
            RouteParserToken::Capture(RefCaptureVariant::ManyNamed("lorem")),
        ];
        let path_matcher = RouteMatcher::from(tokens);
        let (_, _matches) = path_matcher
            .capture_route_into_map("/a/")
            .expect("should parse");
    }

    #[test]
    fn fail_match_with_trailing_match_single() {
        let tokens = vec![
            RouteParserToken::Separator,
            RouteParserToken::Exact("a"),
            RouteParserToken::Separator,
            RouteParserToken::Capture(RefCaptureVariant::Named("lorem")),
        ];
        let path_matcher = RouteMatcher::from(tokens);
        path_matcher
            .capture_route_into_map("/a/")
            .expect_err("should not parse");
    }

    #[test]
    fn match_n() {
        let tokens = vec![
            RouteParserToken::Separator,
            RouteParserToken::Capture(RefCaptureVariant::NumberedNamed {
                sections: 3,
                name: "lorem",
            }),
            RouteParserToken::Separator,
            RouteParserToken::Exact("a"),
        ];
        let path_matcher = RouteMatcher::from(tokens);
        let (_, _matches) = path_matcher
            .capture_route_into_map("/garbage1/garbage2/garbage3/a")
            .expect("should parse");
    }

    #[test]
    fn match_n_no_overrun() {
        let tokens = vec![
            RouteParserToken::Separator,
            RouteParserToken::Capture(RefCaptureVariant::NumberedNamed {
                sections: 3,
                name: "lorem",
            }),
        ];
        let path_matcher = RouteMatcher::from(tokens);
        let (s, _matches) = path_matcher
            .capture_route_into_map("/garbage1/garbage2/garbage3")
            .expect("should parse");
        assert_eq!(s.len(), 0)
    }

    #[test]
    fn match_n_named() {
        let tokens = vec![
            RouteParserToken::Separator,
            RouteParserToken::Capture(RefCaptureVariant::NumberedNamed {
                sections: 3,
                name: "captured",
            }),
            RouteParserToken::Separator,
            RouteParserToken::Exact("a"),
        ];
        let path_matcher = RouteMatcher::from(tokens);
        let (_, matches) = path_matcher
            .capture_route_into_map("/garbage1/garbage2/garbage3/a")
            .expect("should parse");
        assert_eq!(
            matches["captured"],
            "garbage1/garbage2/garbage3".to_string()
        )
    }

    #[test]
    fn match_many() {
        let tokens = vec![
            RouteParserToken::Separator,
            RouteParserToken::Capture(RefCaptureVariant::ManyNamed("lorem")),
            RouteParserToken::Separator,
            RouteParserToken::Exact("a"),
        ];
        let path_matcher = RouteMatcher::from(tokens);
        let (_, _matches) = path_matcher
            .capture_route_into_map("/garbage1/garbage2/garbage3/a")
            .expect("should parse");
    }

    #[test]
    fn match_many_named() {
        let tokens = vec![
            RouteParserToken::Separator,
            RouteParserToken::Capture(RefCaptureVariant::ManyNamed("captured")),
            RouteParserToken::Separator,
            RouteParserToken::Exact("a"),
        ];
        let path_matcher = RouteMatcher::from(tokens);
        let (_, matches) = path_matcher
            .capture_route_into_map("/garbage1/garbage2/garbage3/a")
            .expect("should parse");
        assert_eq!(
            matches["captured"],
            "garbage1/garbage2/garbage3".to_string()
        )
    }
}
