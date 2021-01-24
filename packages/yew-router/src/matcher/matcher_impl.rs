use crate::matcher::{
    util::{consume_until, next_delimiter, tag_possibly_case_sensitive},
    Captures, MatcherSettings,
};
use log::trace;
use nom::{
    bytes::complete::{is_not, tag},
    combinator::map,
    error::ErrorKind,
    sequence::terminated,
    IResult,
};
use std::{iter::Peekable, slice::Iter};
use yew_router_route_parser::{CaptureVariant, MatcherToken};

/// Allows abstracting over capturing into a HashMap (Captures) or a Vec.
trait CaptureCollection<'a> {
    fn new2() -> Self;
    fn insert2(&mut self, key: &'a str, value: String);
    fn extend2(&mut self, other: Self);
}

impl<'a> CaptureCollection<'a> for Captures<'a> {
    fn new2() -> Self {
        Captures::new()
    }

    fn insert2(&mut self, key: &'a str, value: String) {
        self.insert(key, value);
    }

    fn extend2(&mut self, other: Self) {
        self.extend(other)
    }
}

impl<'a> CaptureCollection<'a> for Vec<String> {
    fn new2() -> Self {
        Vec::new()
    }

    fn insert2(&mut self, _key: &'a str, value: String) {
        self.push(value)
    }

    fn extend2(&mut self, other: Self) {
        self.extend(other)
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)]
pub(super) fn match_into_map<'a, 'b: 'a>(
    tokens: &'b [MatcherToken],
    settings: &'b MatcherSettings,
) -> impl Fn(&'a str) -> IResult<&'a str, Captures<'b>> {
    move |i: &str| matcher_impl(tokens, *settings, i)
}

#[allow(clippy::trivially_copy_pass_by_ref)]
pub(super) fn match_into_vec<'a, 'b: 'a>(
    tokens: &'b [MatcherToken],
    settings: &'b MatcherSettings,
) -> impl Fn(&'a str) -> IResult<&'a str, Vec<String>> {
    move |i: &str| matcher_impl(tokens, *settings, i)
}

fn matcher_impl<'a, 'b: 'a, CAP: CaptureCollection<'b>>(
    tokens: &'b [MatcherToken],
    settings: MatcherSettings,
    mut i: &'a str,
) -> IResult<&'a str, CAP> {
    trace!("Attempting to match route: {:?} using: {:?}", i, tokens);

    let mut iter = tokens.iter().peekable();

    let mut captures: CAP = CAP::new2();

    while let Some(token) = iter.next() {
        i = match token {
            MatcherToken::Exact(literal) => {
                trace!("Matching '{}' against literal: '{}'", i, literal);
                tag_possibly_case_sensitive(literal.as_str(), !settings.case_insensitive)(i)?.0
            }
            MatcherToken::Capture(capture) => match &capture {
                CaptureVariant::Named(name) => capture_named(i, &mut iter, &name, &mut captures)?,
                CaptureVariant::ManyNamed(name) => {
                    capture_many_named(i, &mut iter, &name, &mut captures)?
                }
                CaptureVariant::NumberedNamed { sections, name } => {
                    capture_numbered_named(i, &mut iter, Some((&name, &mut captures)), *sections)?
                }
                CaptureVariant::Unnamed => capture_named(i, &mut iter, "", &mut captures)?,
                CaptureVariant::ManyUnnamed => capture_many_named(i, &mut iter, "", &mut captures)?,
                CaptureVariant::NumberedUnnamed { sections } => {
                    capture_numbered_named(i, &mut iter, Some(("", &mut captures)), *sections)?
                }
            },
            MatcherToken::End => {
                if !i.is_empty() {
                    // this is approximately correct, but ultimately doesn't matter
                    return Err(nom::Err::Failure((i, ErrorKind::Eof)));
                } else {
                    i
                }
            }
        };
    }
    trace!("Route Matched");

    Ok((i, captures))
}

fn capture_named<'a, 'b: 'a, CAP: CaptureCollection<'b>>(
    i: &'a str,
    iter: &mut Peekable<Iter<MatcherToken>>,
    capture_key: &'b str,
    matches: &mut CAP,
) -> Result<&'a str, nom::Err<(&'a str, ErrorKind)>> {
    log::trace!("Matching Named ({})", capture_key);
    if let Some(_peaked_next_token) = iter.peek() {
        let delimiter = next_delimiter(iter);
        let (ii, captured) = consume_until(delimiter)(i)?;
        matches.insert2(capture_key, captured);
        Ok(ii)
    } else {
        let (ii, captured) = map(valid_capture_characters, String::from)(i)?;
        matches.insert2(capture_key, captured);
        Ok(ii)
    }
}

fn capture_many_named<'a, 'b, CAP: CaptureCollection<'b>>(
    i: &'a str,
    iter: &mut Peekable<Iter<MatcherToken>>,
    capture_key: &'b str,
    matches: &mut CAP,
) -> Result<&'a str, nom::Err<(&'a str, ErrorKind)>> {
    log::trace!("Matching ManyUnnamed ({})", capture_key);
    if let Some(_peaked_next_token) = iter.peek() {
        let delimiter = next_delimiter(iter);
        let (ii, captured) = consume_until(delimiter)(i)?;
        matches.insert2(&capture_key, captured);
        Ok(ii)
    } else if i.is_empty() {
        // If the route string is empty, return an empty value.
        matches.insert2(&capture_key, "".to_string());
        Ok(i) // Match even if nothing is left
    } else {
        let (ii, c) = map(valid_many_capture_characters, String::from)(i)?;
        matches.insert2(&capture_key, c);
        Ok(ii)
    }
}

fn capture_numbered_named<'a, 'b, CAP: CaptureCollection<'b>>(
    mut i: &'a str,
    iter: &mut Peekable<Iter<MatcherToken>>,
    name_and_captures: Option<(&'b str, &mut CAP)>,
    mut sections: usize,
) -> Result<&'a str, nom::Err<(&'a str, ErrorKind)>> {
    log::trace!("Matching NumberedNamed ({})", sections);
    let mut captured = "".to_string();

    if let Some(_peaked_next_token) = iter.peek() {
        while sections > 0 {
            if sections > 1 {
                let (ii, c) = terminated(valid_capture_characters, tag("/"))(i)?;
                i = ii;
                captured += c;
                captured += "/";
            } else {
                let delimiter = next_delimiter(iter);
                let (ii, c) = consume_until(delimiter)(i)?;
                i = ii;
                captured += &c;
            }
            sections -= 1;
        }
    } else {
        while sections > 0 {
            if sections > 1 {
                let (ii, c) = terminated(valid_capture_characters, tag("/"))(i)?;
                i = ii;
                captured += c;
                captured += "/";
            } else {
                // Don't consume the next character on the last section
                let (ii, c) = valid_capture_characters(i)?;
                i = ii;
                captured += c;
            }
            sections -= 1;
        }
    }

    if let Some((name, captures)) = name_and_captures {
        captures.insert2(&name, captured);
    }
    Ok(i)
}

/// Characters that don't interfere with parsing logic for capturing characters
fn valid_capture_characters(i: &str) -> IResult<&str, &str> {
    const INVALID_CHARACTERS: &str = " */#&?{}=";
    is_not(INVALID_CHARACTERS)(i)
}

fn valid_many_capture_characters(i: &str) -> IResult<&str, &str> {
    const INVALID_CHARACTERS: &str = " #&?=";
    is_not(INVALID_CHARACTERS)(i)
}

#[cfg(test)]
mod integration_test {
    use super::*;

    use yew_router_route_parser::{self, FieldNamingScheme};

    use super::super::Captures;
    //    use nom::combinator::all_consuming;

    #[test]
    fn match_query_after_path() {
        let x = yew_router_route_parser::parse_str_and_optimize_tokens(
            "/a/path?lorem=ipsum",
            FieldNamingScheme::Unnamed,
        )
        .expect("Should parse");
        matcher_impl::<Captures>(&x, MatcherSettings::default(), "/a/path?lorem=ipsum")
            .expect("should match");
    }

    #[test]
    fn match_query_after_path_trailing_slash() {
        let x = yew_router_route_parser::parse_str_and_optimize_tokens(
            "/a/path/?lorem=ipsum",
            FieldNamingScheme::Unnamed,
        )
        .expect("Should parse");
        matcher_impl::<Captures>(&x, MatcherSettings::default(), "/a/path/?lorem=ipsum")
            .expect("should match");
    }

    #[test]
    fn match_query() {
        let x = yew_router_route_parser::parse_str_and_optimize_tokens(
            "?lorem=ipsum",
            FieldNamingScheme::Unnamed,
        )
        .expect("Should parse");
        matcher_impl::<Captures>(&x, MatcherSettings::default(), "?lorem=ipsum")
            .expect("should match");
    }

    #[test]
    fn named_capture_query() {
        let x = yew_router_route_parser::parse_str_and_optimize_tokens(
            "?lorem={ipsum}",
            FieldNamingScheme::Unnamed,
        )
        .expect("Should parse");
        let (_, matches) = matcher_impl::<Captures>(&x, MatcherSettings::default(), "?lorem=ipsum")
            .expect("should match");
        assert_eq!(matches["ipsum"], "ipsum".to_string())
    }

    #[test]
    fn match_n_paths_3() {
        let x = yew_router_route_parser::parse_str_and_optimize_tokens(
            "/{*:cap}/thing",
            FieldNamingScheme::Unnamed,
        )
        .expect("Should parse");
        let matches: Captures =
            matcher_impl(&x, MatcherSettings::default(), "/anything/other/thing")
                .expect("should match")
                .1;
        assert_eq!(matches["cap"], "anything/other".to_string())
    }

    #[test]
    fn match_n_paths_4() {
        let x = yew_router_route_parser::parse_str_and_optimize_tokens(
            "/{*:cap}/thing",
            FieldNamingScheme::Unnamed,
        )
        .expect("Should parse");
        let matches: Captures =
            matcher_impl(&x, MatcherSettings::default(), "/anything/thing/thing")
                .expect("should match")
                .1;
        assert_eq!(matches["cap"], "anything".to_string())
    }

    #[test]
    fn match_path_5() {
        let x = yew_router_route_parser::parse_str_and_optimize_tokens(
            "/{cap}/thing",
            FieldNamingScheme::Unnamed,
        )
        .expect("Should parse");
        let matches: Captures =
            matcher_impl(&x, MatcherSettings::default(), "/anything/thing/thing")
                .expect("should match")
                .1;
        assert_eq!(matches["cap"], "anything".to_string())
    }

    #[test]
    fn match_fragment() {
        let x = yew_router_route_parser::parse_str_and_optimize_tokens(
            "#test",
            FieldNamingScheme::Unnamed,
        )
        .expect("Should parse");
        matcher_impl::<Captures>(&x, MatcherSettings::default(), "#test").expect("should match");
    }

    #[test]
    fn match_fragment_after_path() {
        let x = yew_router_route_parser::parse_str_and_optimize_tokens(
            "/a/path/#test",
            FieldNamingScheme::Unnamed,
        )
        .expect("Should parse");
        matcher_impl::<Captures>(&x, MatcherSettings::default(), "/a/path/#test")
            .expect("should match");
    }

    #[test]
    fn match_fragment_after_path_no_slash() {
        let x = yew_router_route_parser::parse_str_and_optimize_tokens(
            "/a/path#test",
            FieldNamingScheme::Unnamed,
        )
        .expect("Should parse");
        matcher_impl::<Captures>(&x, MatcherSettings::default(), "/a/path#test")
            .expect("should match");
    }

    #[test]
    fn match_fragment_after_query() {
        let x = yew_router_route_parser::parse_str_and_optimize_tokens(
            "/a/path?query=thing#test",
            FieldNamingScheme::Unnamed,
        )
        .expect("Should parse");
        matcher_impl::<Captures>(&x, MatcherSettings::default(), "/a/path?query=thing#test")
            .expect("should match");
    }

    #[test]
    fn match_fragment_after_query_capture() {
        let x = yew_router_route_parser::parse_str_and_optimize_tokens(
            "/a/path?query={capture}#test",
            FieldNamingScheme::Unnamed,
        )
        .expect("Should parse");
        matcher_impl::<Captures>(&x, MatcherSettings::default(), "/a/path?query=thing#test")
            .expect("should match");
    }

    #[test]
    fn capture_as_only_token() {
        let x = yew_router_route_parser::parse_str_and_optimize_tokens(
            "{any}",
            FieldNamingScheme::Unnamed,
        )
        .expect("Should parse");
        matcher_impl::<Captures>(&x, MatcherSettings::default(), "literally_anything")
            .expect("should match");
    }

    #[test]
    fn case_insensitive() {
        let x = yew_router_route_parser::parse_str_and_optimize_tokens(
            "/hello",
            FieldNamingScheme::Unnamed,
        )
        .expect("Should parse");
        let settings = MatcherSettings {
            case_insensitive: true,
        };
        matcher_impl::<Captures>(&x, settings, "/HeLLo").expect("should match");
    }

    #[test]
    fn end_token() {
        let x = yew_router_route_parser::parse_str_and_optimize_tokens(
            "/lorem!",
            FieldNamingScheme::Unnamed,
        )
        .expect("Should parse");

        matcher_impl::<Captures>(&x, Default::default(), "/lorem/ipsum")
            .expect_err("should not match");
    }
}
