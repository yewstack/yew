//! Utility library for the Yew frontend web framework to handle validating strings relating
//! to HTML/SVG/MathML tags.

/// Returns true when the character provided is a "control" as defined
/// in [the WhatWG spec](https://infra.spec.whatwg.org/#control)
fn is_control(c: char) -> bool {
    match c {
        '\u{007F}'..='\u{009F}' => true,
        _ => is_c0_control(c),
    }
}

/// Returns true when the character provided is a "c0 control" as defined
/// in [the WhatWG spec](https://infra.spec.whatwg.org/#c0-control)
fn is_c0_control(c: char) -> bool {
    matches!(c, '\u{0000}'..='\u{001F}')
}

/// Returns true when the string provided is a "noncharacter" as defined
/// in [the WhatWG spec](https://infra.spec.whatwg.org/#noncharacter)
fn is_noncharacter(c: char) -> bool {
    matches!(
        c,
        '\u{FDD0}'
            ..='\u{FDEF}'
                | '\u{FFFE}'
                | '\u{FFFF}'
                | '\u{1FFFE}'
                | '\u{1FFFF}'
                | '\u{2FFFE}'
                | '\u{2FFFF}'
                | '\u{3FFFE}'
                | '\u{3FFFF}'
                | '\u{4FFFE}'
                | '\u{4FFFF}'
                | '\u{5FFFE}'
                | '\u{5FFFF}'
                | '\u{6FFFE}'
                | '\u{6FFFF}'
                | '\u{7FFFE}'
                | '\u{7FFFF}'
                | '\u{8FFFE}'
                | '\u{8FFFF}'
                | '\u{9FFFE}'
                | '\u{9FFFF}'
                | '\u{AFFFE}'
                | '\u{AFFFF}'
                | '\u{BFFFE}'
                | '\u{BFFFF}'
                | '\u{CFFFE}'
                | '\u{CFFFF}'
                | '\u{DFFFE}'
                | '\u{DFFFF}'
                | '\u{EFFFE}'
                | '\u{EFFFF}'
                | '\u{FFFFE}'
                | '\u{FFFFF}'
                | '\u{10FFFE}'
                | '\u{10FFFF}'
    )
}

/// Returns true when the string provided is a valid "attribute name" as defined
/// in [the WhatWG spec](https://html.spec.whatwg.org/multipage/syntax.html#syntax-attribute-name)
pub fn is_valid_html_attribute_name(attr: &str) -> bool {
    for c in attr.chars() {
        if is_noncharacter(c)
            || is_control(c)
            || c == '\u{0020}'
            || c == '\u{0022}'
            || c == '\u{0027}'
            || c == '\u{003E}'
            || c == '\u{002F}'
            || c == '\u{003D}'
        {
            return false;
        }
    }
    true
}

/// Returns true when the character provided is a valid PCENChar as defined
/// in [the WhatWG spec](https://html.spec.whatwg.org/multipage/custom-elements.html#prod-pcenchar)
fn is_pcen_char(c: char) -> bool {
    matches!(c, '-' | '.' | '0'..='9' | 'a'..='z' | '_'
        | '\u{B7}'
        | '\u{C0}'..='\u{D6}'
        | '\u{D8}'..='\u{F6}'
        | '\u{F8}'..='\u{37D}'
        | '\u{37F}'..='\u{1FFF}'
        | '\u{200C}'..='\u{200D}'
        | '\u{203F}'..='\u{2040}'
        | '\u{2070}'..='\u{218F}'
        | '\u{2C00}'..='\u{2FEF}'
        | '\u{3001}'..='\u{D7FF}'
        | '\u{F900}'..='\u{FDCF}'
        | '\u{FDF0}'..='\u{FFFD}'
        | '\u{10000}'..='\u{EFFFF}'
    )
}

/// Returns true when the tag name provided would be a valid "custom element" per
/// [the WhatWG spec](https://html.spec.whatwg.org/multipage/custom-elements.html#valid-custom-element-name).
/// Only technically returns correct results if called with a string that is not one of the following:
///     - annotation-xml
///     - color-profile
///     - font-face
///     - font-face-src
///     - font-face-uri
///     - font-face-format
///     - font-face-name
///     - missing-glyph
/// But, given the way it is used in this file, as of this writing, this limitation does not affect the
/// behavior of the program.
fn is_valid_html_custom_element_name(tag: &str) -> bool {
    let mut chars = tag.chars();
    let first_char = chars.next();

    match first_char {
        None => false,
        Some(first_char) => {
            // must begin with [a-z]
            if !('a'..='z').contains(&first_char) {
                return false;
            }

            let mut seen_hyphen = false;
            for c in chars {
                if c == '-' {
                    seen_hyphen = true
                }

                // all characters must be valid PCENChar's
                if !is_pcen_char(c) {
                    return false;
                }
            }

            // must contain at least one hyphen
            seen_hyphen
        }
    }
}

/// Returns true when the tag name provided looks like a valid non-custom HTML element or valid SVG element.
/// There's no official spec here, it's just arbitrary.
fn resembles_standard_html_element_name(tag: &str) -> bool {
    // must contain at least one character
    if tag.is_empty() {
        return false;
    }

    let mut saw_non_hyphen = false;
    for c in tag.chars() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => saw_non_hyphen = true,
            '-' => {}
            _ => {
                return false;
            }
        }
    }

    saw_non_hyphen
}

/// Returns true when you could validly construct a tag using this name in an HTML document
pub fn is_valid_sgml_tag(tag: &str) -> bool {
    resembles_standard_html_element_name(tag) || is_valid_html_custom_element_name(tag)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_custom_element() {
        assert_eq!(is_valid_html_custom_element_name("foo-bar"), true);
        assert_eq!(is_valid_html_custom_element_name("foo-"), true);
        assert_eq!(is_valid_html_custom_element_name("bar-baz"), true);
    }

    #[test]
    fn invalid_custom_element() {
        assert_eq!(is_valid_html_custom_element_name("foobar"), false);
        assert_eq!(is_valid_html_custom_element_name("-bar"), false);
        assert_eq!(is_valid_html_custom_element_name("foo bar"), false);
        assert_eq!(is_valid_html_custom_element_name(""), false);
        assert_eq!(is_valid_html_custom_element_name("foo\nbar"), false);
        assert_eq!(is_valid_html_custom_element_name("-"), false);
    }

    #[test]
    fn valid_html_element() {
        assert_eq!(resembles_standard_html_element_name("section"), true);
        assert_eq!(resembles_standard_html_element_name("h2"), true);
        assert_eq!(resembles_standard_html_element_name("applet"), true);
        assert_eq!(resembles_standard_html_element_name("appLET"), true);
        assert_eq!(resembles_standard_html_element_name("aPPlet"), true);
        assert_eq!(resembles_standard_html_element_name("foo-bar"), true);
    }

    #[test]
    fn invalid_html_element() {
        assert_eq!(resembles_standard_html_element_name(" foo"), false);
        assert_eq!(resembles_standard_html_element_name("foo "), false);
        assert_eq!(resembles_standard_html_element_name("-"), false);
        assert_eq!(resembles_standard_html_element_name("!doctype"), false);
    }

    #[test]
    fn valid_html_attribute() {
        assert_eq!(is_valid_html_attribute_name("-foo-bar"), true);
        assert_eq!(is_valid_html_attribute_name("data-foobar"), true);
        assert_eq!(is_valid_html_attribute_name("foo<bar"), true); // shocking but true
    }

    #[test]
    fn invalid_html_attribute() {
        assert_eq!(is_valid_html_attribute_name("foo=bar"), false);
        assert_eq!(is_valid_html_attribute_name("\"foo\""), false);
        assert_eq!(is_valid_html_attribute_name("foo bar"), false);
        assert_eq!(is_valid_html_attribute_name("foo>bar"), false);
    }

    #[test]
    fn invalid_sgml_tag() {
        assert_eq!(is_valid_sgml_tag("f>bar"), false);
        assert_eq!(is_valid_sgml_tag("f<bar"), false);
        assert_eq!(is_valid_sgml_tag("/>"), false);
    }
}
