use nom::error::ErrorKind;
use std::fmt;

/// Parser error that can print itself in a human-readable format.
#[derive(Clone, PartialEq)]
pub struct PrettyParseError<'a> {
    /// Inner error
    pub error: ParseError,
    /// Input to the parser
    pub input: &'a str,
    /// Remaining input after partially tokenizing
    pub remaining: &'a str,
}

/// Simple offset calculator to determine where to place the carrot for indicating an error.
fn offset(input: &str, substring: &str) -> usize {
    input.len() - substring.len()
}

impl<'a> fmt::Debug for PrettyParseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Could not parse route.")?;
        f.write_str("\n")?;

        let route_str: &str = "Route: ";
        f.write_str(route_str)?;
        f.write_str(self.input)?;
        f.write_str("\n")?;

        let offset = offset(self.input, self.remaining);
        let offset = offset + self.error.offset;
        let pad = (0..offset + route_str.len())
            .map(|_| '-')
            .collect::<String>();
        f.write_str(&format!("{}^", pad))?;
        f.write_str("\n")?;

        if !self.error.expected.is_empty() {
            f.write_str("Expected: ")?;
            self.error.expected[..self.error.expected.len() - 1]
                .iter()
                .try_for_each(|expected| {
                    <ExpectedToken as fmt::Display>::fmt(expected, f)
                        .and_then(|_| f.write_str(", "))
                })?;
            self.error
                .expected
                .last()
                .map(|expected| <ExpectedToken as fmt::Display>::fmt(expected, f))
                .transpose()?;
            f.write_str("\n")?;
        }

        if let Some(reason) = self.error.reason {
            f.write_str("Reason: ")?;
            <ParserErrorReason as fmt::Display>::fmt(&reason, f)?;
        }

        Ok(())
    }
}

/// Error for parsing the route
#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    /// A concrete reason why the parse failed.
    pub reason: Option<ParserErrorReason>,
    /// Expected token sequences
    pub expected: Vec<ExpectedToken>,
    /// Additional offset for failures within sub-parsers.
    /// Eg. if `{` parses, but then a bad ident is presented, some offset is needed here then.
    pub offset: usize,
}

impl ParseError {
    pub(crate) fn expected(expected: ExpectedToken) -> Self {
        ParseError {
            reason: None,
            expected: vec![expected],
            offset: 0,
        }
    }
}

impl nom::error::ParseError<&str> for ParseError {
    fn from_error_kind(_input: &str, _kind: ErrorKind) -> Self {
        ParseError {
            reason: None,
            expected: vec![],
            offset: 0,
        }
    }

    fn append(_input: &str, _kind: ErrorKind, other: Self) -> Self {
        other
    }

    fn or(mut self, other: Self) -> Self {
        // It is assumed that there aren't duplicates.
        self.expected.extend(other.expected);

        ParseError {
            reason: other.reason.or(self.reason), // Take the right most reason
            expected: self.expected,
            offset: other.offset, /* Defer to the "other"'s offset. TODO it might make sense if the offsets are different, only show the other's "expected". */
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExpectedToken {
    ///  /
    Separator,
    /// specific string.
    Literal,
    ///  ?
    QueryBegin,
    ///  &
    QuerySeparator,
    /// \#
    FragmentBegin,
    /// !
    End,
    /// identifier within {}
    Ident,
    /// {
    OpenBracket,
    /// }
    CloseBracket,
    /// =
    Equals,
    /// *
    Star,
    /// :
    Colon,
}

impl fmt::Display for ExpectedToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpectedToken::Separator => f.write_str("/"),
            ExpectedToken::Literal => f.write_str("<literal>"),
            ExpectedToken::QueryBegin => f.write_str("?"),
            ExpectedToken::QuerySeparator => f.write_str("&"),
            ExpectedToken::FragmentBegin => f.write_str("#"),
            ExpectedToken::End => f.write_str("!"),
            ExpectedToken::Ident => f.write_str("<ident>"),
            ExpectedToken::OpenBracket => f.write_str("{"),
            ExpectedToken::CloseBracket => f.write_str("}"),
            ExpectedToken::Equals => f.write_str("="),
            ExpectedToken::Star => f.write_str("*"),
            ExpectedToken::Colon => f.write_str(":"),
        }
    }
}

/// A concrete reason why a parse failed
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParserErrorReason {
    /// Some token encountered after the end token.
    TokensAfterEndToken,
    /// Two slashes are able to occur next to each other.
    DoubleSlash,
    /// End after a {}
    EndAfterCapture,
    /// A & appears before a ?
    AndBeforeQuestion,
    /// Captures can't be next to each other
    AdjacentCaptures,
    /// There can only be one question mark in the query section
    MultipleQuestions,
    /// The provided ident within a capture group could never match with a valid rust identifier.
    BadRustIdent(char),
    /// A bad literal.
    BadLiteral,
    /// Invalid state
    InvalidState,
    /// Can't have capture sections for unit structs/variants
    CapturesInUnit,
    /// Internal check on valid state transitions
    /// This should never actually be created.
    NotAllowedStateTransition,
    /// Expected a specific token
    Expected(ExpectedToken),
}

impl fmt::Display for ParserErrorReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserErrorReason::TokensAfterEndToken => {
                f.write_str("Characters appeared after the end token (!).")?;
            }
            ParserErrorReason::DoubleSlash => {
                f.write_str("Two slashes are not allowed to be next to each other (//).")?;
            }
            ParserErrorReason::AndBeforeQuestion => {
                f.write_str("The first query must be indicated with a '?', not a '&'.")?;
            }
            ParserErrorReason::AdjacentCaptures => {
                f.write_str("Capture groups can't be next to each other. There must be some character in between the '}' and '{' characters.")?;
            }
            ParserErrorReason::InvalidState => {
                f.write_str("Library Error: The parser was able to enter into an invalid state.")?;
            }
            ParserErrorReason::NotAllowedStateTransition => {
                f.write_str("Library Error: A state transition was attempted that would put the parser in an invalid state")?;
            }
            ParserErrorReason::MultipleQuestions => {
                f.write_str("There can only be one question mark in the query section. `&` should be used to separate other queries.")?;
            }
            ParserErrorReason::BadRustIdent(c) => {
                f.write_str(&format!(
                    "The character: '{}' could not be used as a Rust identifier.",
                    c
                ))?;
            }
            ParserErrorReason::EndAfterCapture => {
                f.write_str("The end token (!) can't appear after a capture ({}).")?;
            }
            ParserErrorReason::Expected(expected) => {
                f.write_str(&format!("Expected: {}", expected))?;
            }
            ParserErrorReason::BadLiteral => {
                f.write_str("Malformed literal.")?;
            }
            ParserErrorReason::CapturesInUnit => {
                f.write_str("Cannot have a capture section for a unit struct or variant.")?;
            }
        }
        Ok(())
    }
}

pub(crate) fn get_reason(err: &mut nom::Err<ParseError>) -> &mut Option<ParserErrorReason> {
    match err {
        nom::Err::Error(err) | nom::Err::Failure(err) => &mut err.reason,
        nom::Err::Incomplete(_) => panic!("Incomplete not possible"),
    }
}
