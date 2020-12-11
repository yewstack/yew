use crate::{
    error::{ExpectedToken, ParserErrorReason},
    parser::{CaptureOrExact, RefCaptureVariant, RouteParserToken},
    ParseError,
};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1},
    character::{
        complete::{char, digit1},
        is_digit,
    },
    combinator::{map, map_parser},
    error::ErrorKind,
    sequence::{delimited, separated_pair},
    IResult,
};

/// Indicates if the parser is working to create a matcher for a datastructure with named or unnamed fields.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub enum FieldNamingScheme {
    /// For Thing { field: String }
    Named,
    /// for Thing(String)
    Unnamed,
    /// for Thing
    Unit,
}

pub fn get_slash(i: &str) -> IResult<&str, RouteParserToken, ParseError> {
    map(char('/'), |_: char| RouteParserToken::Separator)(i)
        .map_err(|_: nom::Err<()>| nom::Err::Error(ParseError::expected(ExpectedToken::Separator)))
}

pub fn get_question(i: &str) -> IResult<&str, RouteParserToken, ParseError> {
    map(char('?'), |_: char| RouteParserToken::QueryBegin)(i)
        .map_err(|_: nom::Err<()>| nom::Err::Error(ParseError::expected(ExpectedToken::QueryBegin)))
}

pub fn get_and(i: &str) -> IResult<&str, RouteParserToken, ParseError> {
    map(char('&'), |_: char| RouteParserToken::QuerySeparator)(i).map_err(|_: nom::Err<()>| {
        nom::Err::Error(ParseError::expected(ExpectedToken::QuerySeparator))
    })
}

/// Returns a FragmentBegin variant if the next character is '\#'.
pub fn get_hash(i: &str) -> IResult<&str, RouteParserToken, ParseError> {
    map(char('#'), |_: char| RouteParserToken::FragmentBegin)(i).map_err(|_: nom::Err<()>| {
        nom::Err::Error(ParseError::expected(ExpectedToken::FragmentBegin))
    })
}

/// Returns an End variant if the next character is a '!`.
pub fn get_end(i: &str) -> IResult<&str, RouteParserToken, ParseError> {
    map(char('!'), |_: char| RouteParserToken::End)(i)
        .map_err(|_: nom::Err<()>| nom::Err::Error(ParseError::expected(ExpectedToken::End)))
}

/// Returns an End variant if the next character is a '!`.
fn get_open_bracket(i: &str) -> IResult<&str, (), ParseError> {
    map(char('{'), |_: char| ())(i).map_err(|_: nom::Err<()>| {
        nom::Err::Error(ParseError::expected(ExpectedToken::OpenBracket))
    })
}

fn get_close_bracket(i: &str) -> IResult<&str, (), ParseError> {
    map(char('}'), |_: char| ())(i).map_err(|_: nom::Err<()>| {
        nom::Err::Error(ParseError::expected(ExpectedToken::CloseBracket))
    })
}

fn get_eq(i: &str) -> IResult<&str, (), ParseError> {
    map(char('='), |_: char| ())(i)
        .map_err(|_: nom::Err<()>| nom::Err::Error(ParseError::expected(ExpectedToken::Equals)))
}

fn get_star(i: &str) -> IResult<&str, (), ParseError> {
    map(char('*'), |_: char| ())(i)
        .map_err(|_: nom::Err<()>| nom::Err::Error(ParseError::expected(ExpectedToken::Star)))
}

fn get_colon(i: &str) -> IResult<&str, (), ParseError> {
    map(char(':'), |_: char| ())(i)
        .map_err(|_: nom::Err<()>| nom::Err::Error(ParseError::expected(ExpectedToken::Colon)))
}

fn rust_ident(i: &str) -> IResult<&str, &str, ParseError> {
    let invalid_ident_chars = r##" \|/{[]()?+=-!@#$%^&*~`'";:"##;
    // Detect an ident by first reading until a } is found,
    // then validating the captured section against invalid characters that can't be in rust idents.
    map_parser(take_till1(move |c| c == '}'), move |i: &str| {
        match take_till1::<_, _, ()>(|c| invalid_ident_chars.contains(c))(i) {
            Ok((remain, got)) => {
                // Detects if the first character is a digit.
                if !got.is_empty() && got.starts_with(|c: char| is_digit(c as u8)) {
                    Err(nom::Err::Failure(ParseError {
                        reason: Some(ParserErrorReason::BadRustIdent(got.chars().next().unwrap())),
                        expected: vec![ExpectedToken::Ident],
                        offset: 1,
                    }))
                } else if !remain.is_empty() {
                    Err(nom::Err::Failure(ParseError {
                        reason: Some(ParserErrorReason::BadRustIdent(
                            remain.chars().next().unwrap(),
                        )),
                        expected: vec![ExpectedToken::CloseBracket, ExpectedToken::Ident],
                        offset: got.len() + 1,
                    }))
                } else {
                    Ok((i, i))
                }
            }
            Err(_) => Ok((i, i)),
        }
    })(i)
}

/// Matches escaped items
fn escaped_item_impl(i: &str) -> IResult<&str, &str> {
    map(alt((tag("!!"), tag("{{"), tag("}}"))), |s| match s {
        "!!" => "!",
        "}}" => "}",
        "{{" => "{",
        _ => unreachable!(),
    })(i)
}

/// Matches "".
pub fn nothing(i: &str) -> IResult<&str, RouteParserToken, ParseError> {
    if i.is_empty() {
        Ok((i, RouteParserToken::Nothing))
    } else {
        Err(nom::Err::Error(ParseError {
            reason: None, // This should never actually report an error.
            expected: vec![],
            offset: 0,
        }))
    }
}

/// The provided string of special characters will be used to terminate this parser.
///
/// Due to escaped character parser, the list of special characters MUST contain the characters:
/// "!{}" within it.
fn exact_impl(special_chars: &'static str) -> impl Fn(&str) -> IResult<&str, &str, ParseError> {
    // Detect either an exact ident, or an escaped item.
    // At higher levels, this can be called multiple times in a row,
    // and that results of those multiple parse attempts will be stitched together into one literal.
    move |i: &str| {
        alt((
            take_till1(move |c| special_chars.contains(c)),
            escaped_item_impl,
        ))(i)
        .map_err(|x: nom::Err<(&str, ErrorKind)>| {
            let s = match x {
                nom::Err::Error((s, _)) | nom::Err::Failure((s, _)) => s,
                nom::Err::Incomplete(_) => panic!(),
            };
            nom::Err::Error(ParseError {
                reason: Some(ParserErrorReason::BadLiteral),
                expected: vec![ExpectedToken::Literal],
                offset: 1 + i.len() - s.len(),
            })
        })
    }
}

const SPECIAL_CHARS: &str = r##"/?&#={}!"##;
const FRAGMENT_SPECIAL_CHARS: &str = r##"{}!"##;

pub fn exact(i: &str) -> IResult<&str, RouteParserToken, ParseError> {
    map(exact_impl(SPECIAL_CHARS), RouteParserToken::Exact)(i)
}

/// More permissive exact matchers
pub fn fragment_exact(i: &str) -> IResult<&str, RouteParserToken, ParseError> {
    map(exact_impl(FRAGMENT_SPECIAL_CHARS), RouteParserToken::Exact)(i)
}

pub fn capture<'a>(
    field_naming_scheme: FieldNamingScheme,
) -> impl Fn(&'a str) -> IResult<&'a str, RouteParserToken<'a>, ParseError> {
    map(capture_impl(field_naming_scheme), RouteParserToken::Capture)
}

fn capture_single_impl<'a>(
    field_naming_scheme: FieldNamingScheme,
) -> impl Fn(&'a str) -> IResult<&'a str, RefCaptureVariant<'a>, ParseError> {
    move |i: &str| match field_naming_scheme {
        FieldNamingScheme::Named => delimited(
            get_open_bracket,
            named::single_capture_impl,
            get_close_bracket,
        )(i),
        FieldNamingScheme::Unnamed => delimited(
            get_open_bracket,
            alt((named::single_capture_impl, unnamed::single_capture_impl)),
            get_close_bracket,
        )(i),
        FieldNamingScheme::Unit => {
            println!("Unit encountered, erroring in capture single");
            Err(nom::Err::Failure(ParseError {
                reason: Some(ParserErrorReason::CapturesInUnit),
                expected: vec![],
                offset: 0,
            }))
        }
    }
}

/// Captures {ident}, {*:ident}, {<number>:ident}
///
/// Depending on the provided field naming, it may also match {}, {*}, and {<number>} for unnamed fields, or none at all for units.
fn capture_impl<'a>(
    field_naming_scheme: FieldNamingScheme,
) -> impl Fn(&'a str) -> IResult<&'a str, RefCaptureVariant, ParseError> {
    move |i: &str| match field_naming_scheme {
        FieldNamingScheme::Named => {
            let inner = alt((
                named::many_capture_impl,
                named::numbered_capture_impl,
                named::single_capture_impl,
            ));
            delimited(get_open_bracket, inner, get_close_bracket)(i)
        }
        FieldNamingScheme::Unnamed => {
            let inner = alt((
                named::many_capture_impl,
                unnamed::many_capture_impl,
                named::numbered_capture_impl,
                unnamed::numbered_capture_impl,
                named::single_capture_impl,
                unnamed::single_capture_impl,
            ));
            delimited(get_open_bracket, inner, get_close_bracket)(i)
        }
        FieldNamingScheme::Unit => Err(nom::Err::Error(ParseError {
            reason: Some(ParserErrorReason::CapturesInUnit),
            expected: vec![],
            offset: 0,
        })),
    }
}

mod named {
    use super::*;
    pub fn single_capture_impl(i: &str) -> IResult<&str, RefCaptureVariant, ParseError> {
        map(rust_ident, |key| RefCaptureVariant::Named(key))(i)
    }

    pub fn many_capture_impl(i: &str) -> IResult<&str, RefCaptureVariant, ParseError> {
        map(
            separated_pair(get_star, get_colon, rust_ident),
            |(_, key)| RefCaptureVariant::ManyNamed(key),
        )(i)
    }

    pub fn numbered_capture_impl(i: &str) -> IResult<&str, RefCaptureVariant, ParseError> {
        map(
            separated_pair(digit1, get_colon, rust_ident),
            |(number, key)| RefCaptureVariant::NumberedNamed {
                sections: number.parse().unwrap(),
                name: key,
            },
        )(i)
    }
}

mod unnamed {
    use super::*;

    /// #Note
    /// because this always succeeds, try this last
    pub fn single_capture_impl(i: &str) -> IResult<&str, RefCaptureVariant, ParseError> {
        Ok((i, RefCaptureVariant::Unnamed))
    }

    pub fn many_capture_impl(i: &str) -> IResult<&str, RefCaptureVariant, ParseError> {
        map(get_star, |_| RefCaptureVariant::ManyUnnamed)(i)
    }

    pub fn numbered_capture_impl(i: &str) -> IResult<&str, RefCaptureVariant, ParseError> {
        map(digit1, |number: &str| RefCaptureVariant::NumberedUnnamed {
            sections: number.parse().unwrap(),
        })(i)
    }
}

/// Gets a capture or exact, mapping it to the CaptureOrExact enum - to provide a limited subset.
fn cap_or_exact<'a>(
    field_naming_scheme: FieldNamingScheme,
) -> impl Fn(&'a str) -> IResult<&'a str, CaptureOrExact<'a>, ParseError> {
    move |i: &str| {
        alt((
            map(
                capture_single_impl(field_naming_scheme),
                CaptureOrExact::Capture,
            ),
            map(exact_impl(SPECIAL_CHARS), CaptureOrExact::Exact),
        ))(i)
    }
}

/// Matches a query
pub fn query<'a>(
    field_naming_scheme: FieldNamingScheme,
) -> impl Fn(&'a str) -> IResult<&'a str, RouteParserToken<'a>, ParseError> {
    move |i: &str| {
        map(
            separated_pair(
                exact_impl(SPECIAL_CHARS),
                get_eq,
                cap_or_exact(field_naming_scheme),
            ),
            |(ident, capture_or_exact)| RouteParserToken::Query {
                ident,
                capture_or_exact,
            },
        )(i)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lit() {
        let x = exact("hello").expect("Should parse");
        assert_eq!(x.1, RouteParserToken::Exact("hello"))
    }

    #[test]
    fn cap_or_exact_match_lit() {
        cap_or_exact(FieldNamingScheme::Named)("lorem").expect("Should parse");
    }
    #[test]
    fn cap_or_exact_match_cap() {
        cap_or_exact(FieldNamingScheme::Named)("{lorem}").expect("Should parse");
    }

    #[test]
    fn query_section_exact() {
        query(FieldNamingScheme::Named)("lorem=ipsum").expect("should parse");
    }

    #[test]
    fn query_section_capture_named() {
        query(FieldNamingScheme::Named)("lorem={ipsum}").expect("should parse");
    }
    #[test]
    fn query_section_capture_named_fails_without_key() {
        query(FieldNamingScheme::Named)("lorem={}").expect_err("should not parse");
    }
    #[test]
    fn query_section_capture_unnamed_succeeds_without_key() {
        query(FieldNamingScheme::Unnamed)("lorem={}").expect("should parse");
    }

    #[test]
    fn non_leading_numbers_in_ident() {
        rust_ident("hello5").expect("sholud parse");
    }
    #[test]
    fn leading_numbers_in_ident_fails() {
        rust_ident("5hello").expect_err("sholud not parse");
    }
}
