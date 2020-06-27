//! This module contains utilities for parsing or validating strings relating
//! to tags.

use lazy_static::lazy_static;

// Source: https://developer.mozilla.org/en-US/docs/Web/HTML/Element
static CONTEMPORARY_HTML_TAGS: [&str; 108] = [
    "a",
    "abbr",
    "address",
    "area",
    "article",
    "aside",
    "audio",
    "b",
    "bdi",
    "bdo",
    "blockquote",
    "body",
    "br",
    "button",
    "canvas",
    "caption",
    "cite",
    "code",
    "col",
    "colgroup",
    "data",
    "datalist",
    "dd",
    "del",
    "details",
    "dfn",
    "dialog",
    "div",
    "dl",
    "dt",
    "em",
    "embed",
    "fieldset",
    "figcaption",
    "figure",
    "footer",
    "form",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "header",
    "hgroup",
    "hr",
    "i",
    "iframe",
    "img",
    "input",
    "ins",
    "kbd",
    "label",
    "legend",
    "li",
    "main",
    "main",
    "map",
    "mark",
    "menu",
    "meter",
    "nav",
    "noscript",
    "object",
    "ol",
    "optgroup",
    "option",
    "output",
    "p",
    "param",
    "picture",
    "pre",
    "progress",
    "q",
    "rb",
    "rp",
    "rt",
    "rtc",
    "ruby",
    "s",
    "samp",
    "script",
    "section",
    "select",
    "slot",
    "small",
    "source",
    "span",
    "strong",
    "sub",
    "summary",
    "sup",
    "table",
    "tbody",
    "td",
    "template",
    "textarea",
    "tfoot",
    "th",
    "thead",
    "time",
    "tr",
    "track",
    "u",
    "ul",
    "var",
    "video",
    "wbr",
];

// Source: https://developer.mozilla.org/en-US/docs/Web/HTML/Element
static DEPRECATED_HTML_TAGS: [&str; 31] = [
    "acronym",
    "applet",
    "basefont",
    "bgsound",
    "big",
    "blink",
    "center",
    "command",
    "content",
    "dir",
    "element",
    "font",
    "frame",
    "frameset",
    "image",
    "isindex",
    "keygen",
    "listing",
    "marquee",
    "menuitem",
    "multicol",
    "nextid",
    "nobr",
    "noembed",
    "noframes",
    "plaintext",
    "shadow",
    "spacer",
    "strike",
    "tt",
    "xmp",
];

// Source: https://developer.mozilla.org/en-US/docs/Web/MathML/Element
static MATHML_TAGS: [&str; 44] = [
    "annotation-xml",
    "annotation",
    "maction",
    "maligngroup",
    "malignmark",
    "math",
    "menclose",
    "merror",
    "mfenced",
    "mfrac",
    "mglyph",
    "mi",
    "mlabeledtr",
    "mlongdiv",
    "mmultiscripts",
    "mn",
    "mo",
    "mover",
    "mpadded",
    "mphantom",
    "mprescripts",
    "mroot",
    "mrow",
    "ms",
    "mscarries",
    "mscarry",
    "msgroup",
    "msline",
    "mspace",
    "msqrt",
    "msrow",
    "mstack",
    "mstyle",
    "msub",
    "msubsup",
    "msup",
    "mtable",
    "mtd",
    "mtext",
    "mtr",
    "munder",
    "munderover",
    "none",
    "semantics",
];

// Source: https://developer.mozilla.org/en-US/docs/Web/SVG/Element
static SVG_TAGS: [&str; 90] = [
    "a",
    "altGlyph",
    "altGlyphDef",
    "altGlyphItem",
    "animate",
    "animateColor",
    "animateMotion",
    "animateTransform",
    "circle",
    "clipPath",
    "color-profile",
    "cursor",
    "defs",
    "desc",
    "discard",
    "ellipse",
    "feBlend",
    "feColorMatrix",
    "feComponentTransfer",
    "feComposite",
    "feConvolveMatrix",
    "feDiffuseLighting",
    "feDisplacementMap",
    "feDistantLight",
    "feDropShadow",
    "feFlood",
    "feFuncA",
    "feFuncB",
    "feFuncG",
    "feFuncR",
    "feGaussianBlur",
    "feImage",
    "feMerge",
    "feMergeNode",
    "feMorphology",
    "feOffset",
    "fePointLight",
    "feSpecularLighting",
    "feSpotLight",
    "feTile",
    "feTurbulence",
    "filter",
    "font-face-format",
    "font-face-name",
    "font-face-src",
    "font-face-uri",
    "font-face",
    "font",
    "foreignObject",
    "g",
    "glyph",
    "glyphRef",
    "hatch",
    "hatchpath",
    "hkern",
    "image",
    "line",
    "linearGradient",
    "marker",
    "mask",
    "mesh",
    "meshgradient",
    "meshpatch",
    "meshrow",
    "metadata",
    "missing-glyph",
    "mpath",
    "path",
    "pattern",
    "polygon",
    "polyline",
    "radialGradient",
    "rect",
    "script",
    "set",
    "solidcolor",
    "stop",
    "style",
    "svg",
    "switch",
    "symbol",
    "text",
    "textPath",
    "title",
    "tref",
    "tspan",
    "unknown",
    "use",
    "view",
    "vkern"
];

lazy_static! {
    static ref DISALLOWED_CUSTOM_ELEMENT_TAGS: Box<[&'static str]> = {
        SVG_TAGS
        .iter()
        .chain(MATHML_TAGS.iter())
        .filter(|tag| tag.contains('-'))
        .map(|t| *t)
        .collect()
    };
}

/// Returns true iff the character provided is a valid PCENChar as defined
/// in the WhatWG spec: https://html.spec.whatwg.org/multipage/custom-elements.html#prod-pcenchar
fn is_valid_pcen_char(c: char) -> bool {
    match c {
        '-' | '.' | '0'..='9' | 'a'..='z' | '_' => true,
        '\u{B7}' => true,
        '\u{C0}'..='\u{D6}' => true,
        '\u{D8}'..='\u{F6}' => true,
        '\u{F8}'..='\u{37D}' => true,
        '\u{37F}'..='\u{1FFF}' => true,
        '\u{200C}'..='\u{200D}' => true,
        '\u{203F}'..='\u{2040}' => true,
        '\u{2070}'..='\u{218F}' => true,
        '\u{2C00}'..='\u{2FEF}' => true,
        '\u{3001}'..='\u{D7FF}' => true,
        '\u{F900}'..='\u{FDCF}' => true,
        '\u{FDF0}'..='\u{FFFD}' => true,
        '\u{10000}'..='\u{EFFFF}' => true,
        _ => false,
    }
}

/// Returns true iff the tag name provided would be a valid "custom element" per
/// WhatWG spec: https://html.spec.whatwg.org/multipage/custom-elements.html#valid-custom-element-name
fn is_valid_custom_element_name(tag: &str) -> bool {
    if (*DISALLOWED_CUSTOM_ELEMENT_TAGS).contains(&tag) {
        return false
    }

    match tag {
        "annotation-xml" | "color-profile" | "font-face" | "font-face-src" | "font-face-uri"
        | "font-face-format" | "font-face-name" | "missing-glyph" => false,
        _ => {
            let mut chars = tag.chars();
            let first_char = chars.next();

            if let None = first_char {
                return false;
            }

            let first_char = first_char.unwrap();

            // must begin with [a-z]
            if first_char < 'a' || first_char > 'z' {
                return false;
            }

            let mut seen_hyphen = false;
            for c in chars {
                if c == '-' {
                    seen_hyphen = true
                }

                // all characters must be valid PCENChar's
                if !is_valid_pcen_char(c) {
                    return false;
                }
            }

            // must contain at least one hyphen
            return seen_hyphen;
        }
    }
}

/// Returns true iff the tag name provided would be a valid HTML element
fn is_valid_html_element_name(tag: &str) -> bool {
    CONTEMPORARY_HTML_TAGS.contains(&tag) || DEPRECATED_HTML_TAGS.contains(&tag)
}

/// Returns true iff the tag name provided would be a valid SVG element
fn is_valid_svg_element_name(tag: &str) -> bool {
    SVG_TAGS.contains(&tag)
}

/// Returns true iff the tag name provided would be a valid MathML element
fn is_valid_mathml_element_name(tag: &str) -> bool {
    MATHML_TAGS.contains(&tag)
}

pub fn is_valid_sgml_tag(tag: &str) -> bool {
    is_valid_html_element_name(tag) ||
    is_valid_svg_element_name(tag) ||
    is_valid_mathml_element_name(tag) ||
    is_valid_custom_element_name(tag)
}