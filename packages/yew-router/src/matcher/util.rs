use nom::{
    bytes::complete::{tag, tag_no_case},
    character::complete::anychar,
    combinator::{cond, map, peek, rest},
    error::{ErrorKind, ParseError},
    multi::many_till,
    sequence::pair,
    IResult,
};
use std::{iter::Peekable, rc::Rc, slice::Iter};
use yew_router_route_parser::MatcherToken;

/// Allows a configurable tag that can optionally be case insensitive.
pub fn tag_possibly_case_sensitive<'a, 'b: 'a>(
    text: &'b str,
    is_sensitive: bool,
) -> impl Fn(&'a str) -> IResult<&'a str, &'a str> {
    map(
        pair(
            cond(is_sensitive, tag(text)),
            cond(!is_sensitive, tag_no_case(text)),
        ),
        |(x, y): (Option<&str>, Option<&str>)| x.xor(y).unwrap(),
    )
}

/// Similar to alt, but works on a vector of tags.
#[allow(unused)]
pub fn alternative(alternatives: Vec<String>) -> impl Fn(&str) -> IResult<&str, &str> {
    move |i: &str| {
        for alternative in &alternatives {
            if let done @ IResult::Ok(..) = tag(alternative.as_str())(i) {
                return done;
            }
        }
        Err(nom::Err::Error((i, ErrorKind::Tag))) // nothing found.
    }
}

/// Consumes the input until the provided parser succeeds.
/// The consumed input is returned in the form of an allocated string.
/// # Note
/// `stop_parser` only peeks its input.
pub fn consume_until<'a, F, E>(stop_parser: F) -> impl Fn(&'a str) -> IResult<&'a str, String, E>
where
    E: ParseError<&'a str>,
    F: Fn(&'a str) -> IResult<&'a str, &'a str, E>,
{
    // In order for the returned fn to be Fn instead of FnOnce, wrap the inner fn in an RC.
    let f = Rc::new(many_till(
        anychar,
        peek(stop_parser), // once this succeeds, stop folding.
    ));
    move |i: &str| {
        let (i, (first, _stop)): (&str, (Vec<char>, &str)) = (f)(i)?;
        let ret = first.into_iter().collect();
        Ok((i, ret))
    }
}

/// Produces a parser combinator that searches for the next possible set of strings of
/// characters used to terminate a forward search.
///
/// # Panics
/// This function assumes that the next item after a Capture must be an Exact.
/// If this is violated, this function will panic.
pub fn next_delimiter<'a>(
    iter: &mut Peekable<Iter<MatcherToken>>,
) -> impl Fn(&'a str) -> IResult<&'a str, &'a str> {
    let t: MatcherToken = iter
        .peek()
        .copied()
        .cloned()
        .expect("There must be at least one token to peak in next_delimiter");

    move |i: &'a str| match &t {
        MatcherToken::Exact(sequence) => tag(sequence.as_str())(i),
        MatcherToken::End => rest(i),
        MatcherToken::Capture(_) => {
            panic!("underlying parser should not allow two captures in a row")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consume_until_simple() {
        let parser = consume_until::<_, ()>(tag("z"));
        let parsed = parser("abcz").expect("Should parse");
        assert_eq!(parsed, ("z", "abc".to_string()))
    }

    #[test]
    fn consume_until_fail() {
        let parser = consume_until(tag("z"));
        let e = parser("abc").expect_err("Should parse");
        assert_eq!(e, nom::Err::Error(("", ErrorKind::Eof)))
    }

    #[test]
    fn alternative_simple() {
        let parser = alternative(
            vec!["c", "d", "abc"]
                .into_iter()
                .map(String::from)
                .collect(),
        );
        let parsed = parser("abcz").expect("Should parse");
        assert_eq!(parsed, ("z", "abc"))
    }

    #[test]
    fn alternative_and_consume_until() {
        let parser = consume_until(alternative(
            vec!["c", "d", "abc"]
                .into_iter()
                .map(String::from)
                .collect(),
        ));
        let parsed = parser("first_stuff_abc").expect("should parse");
        assert_eq!(parsed, ("abc", "first_stuff_".to_string()))
    }

    #[test]
    fn case_sensitive() {
        let parser = tag_possibly_case_sensitive("lorem", true);
        parser("lorem").expect("Should match");
        parser("LoReM").expect_err("Should not match");
    }

    #[test]
    fn case_insensitive() {
        let parser = tag_possibly_case_sensitive("lorem", false);
        parser("lorem").expect("Should match");
        parser("LoREm").expect("Should match");
    }
}
