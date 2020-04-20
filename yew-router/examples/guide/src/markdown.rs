/// Original author of this code is [Nathan Ringo](https://github.com/remexre)
/// Source: https://github.com/acmumn/mentoring/blob/master/web-client/src/view/markdown.rs
use pulldown_cmark::{Alignment, Event, Options, Parser, Tag};
use yew::{
    html,
    virtual_dom::{VNode, VTag, VText},
    Html,
};

/// Renders a string of Markdown to HTML with the default options (footnotes
/// disabled, tables enabled).
pub fn render_markdown(src: &str) -> Html {
    let mut elems = vec![];
    let mut spine = vec![];

    macro_rules! add_child {
        ($child:expr) => {{
            let l = spine.len();
            assert_ne!(l, 0);
            spine[l - 1].add_child($child);
        }};
    }

    let options = Options::ENABLE_TABLES;

    for ev in Parser::new_ext(src, options) {
        match ev {
            Event::Start(tag) => {
                spine.push(make_tag(tag));
            }
            Event::End(tag) => {
                // TODO Verify stack end.
                let l = spine.len();
                assert!(l >= 1);
                let mut top = spine.pop().unwrap();
                if let Tag::CodeBlock(_) = tag {
                    let mut pre = VTag::new("pre");
                    pre.add_child(top.into());
                    top = pre;
                } else if let Tag::Table(aligns) = tag {
                    for r in top.children.iter_mut() {
                        if let VNode::VTag(ref mut vtag) = *r {
                            for (i, c) in vtag.children.iter_mut().enumerate() {
                                if let VNode::VTag(ref mut vtag) = *c {
                                    match aligns[i] {
                                        Alignment::None => {}
                                        Alignment::Left => vtag.add_class("text-left"),
                                        Alignment::Center => vtag.add_class("text-center"),
                                        Alignment::Right => vtag.add_class("text-right"),
                                    }
                                }
                            }
                        }
                    }
                } else if let Tag::TableHead = tag {
                    for c in top.children.iter_mut() {
                        if let VNode::VTag(ref mut vtag) = *c {
                            // TODO
                            //                            vtag.tag = "th".into();
                            vtag.add_attribute("scope", &"col");
                        }
                    }
                }
                if l == 1 {
                    elems.push(top);
                } else {
                    spine[l - 2].add_child(top.into());
                }
            }
            Event::Text(text) => add_child!(VText::new(text.to_string()).into()),
            Event::SoftBreak => add_child!(VText::new("\n".to_string()).into()),
            Event::HardBreak => add_child!(VTag::new("br").into()),
            Event::Code(code) => {
                let mut c = VTag::new("code");
                c.add_child(VText::new(code.to_string()).into());
                add_child!(c.into());
            }
            _ => println!("Unknown event: {:#?}", ev),
        }
    }

    if elems.len() == 1 {
        VNode::VTag(Box::new(elems.pop().unwrap()))
    } else {
        html! {
            <div>{ for elems.into_iter() }</div>
        }
    }
}

fn make_tag(t: Tag) -> VTag {
    match t {
        Tag::Paragraph => VTag::new("p"),
        Tag::BlockQuote => {
            let mut el = VTag::new("blockquote");
            el.add_class("blockquote");
            el
        }
        Tag::CodeBlock(lang) => {
            let mut el = VTag::new("code");
            // Different color schemes may be used for different code blocks,
            // but a different library (likely js based at the moment) would be necessary to
            // actually provide the highlighting support by locating the language
            // classes and applying dom transforms on their contents.
            match lang.as_ref() {
                "html" => el.add_class("html-language"),
                "rust" => el.add_class("rust-language"),
                "java" => el.add_class("java-language"),
                "c" => el.add_class("c-language"),
                _ => {} // Add your own language highlighting support
            };
            el
        }
        Tag::List(None) => VTag::new("ul"),
        Tag::List(Some(1)) => VTag::new("ol"),
        Tag::List(Some(ref start)) => {
            let mut el = VTag::new("ol");
            el.add_attribute("start", start);
            el
        }
        Tag::Item => VTag::new("li"),
        Tag::Table(_) => {
            let mut el = VTag::new("table");
            el.add_class("table");
            el
        }
        Tag::TableHead => VTag::new("tr"),
        Tag::TableRow => VTag::new("tr"),
        Tag::TableCell => VTag::new("td"),
        Tag::Emphasis => {
            let mut el = VTag::new("span");
            el.add_class("font-italic");
            el
        }
        Tag::Strong => {
            let mut el = VTag::new("span");
            el.add_class("font-weight-bold");
            el
        }
        Tag::Link(_lt, ref href, ref title) => {
            let mut el = VTag::new("a");
            el.add_attribute("href", href);
            if title.as_ref() != "" {
                el.add_attribute("title", title);
            }
            el
        }
        Tag::Image(_lt, ref src, ref title) => {
            let mut el = VTag::new("img");
            el.add_attribute("src", src);
            if title.as_ref() != "" {
                el.add_attribute("title", title);
            }
            el
        }

        Tag::FootnoteDefinition(ref _footnote_id) => VTag::new("span"),
        Tag::Strikethrough => VTag::new("strike"),
        Tag::Heading(n) => {
            assert!(n > 0);
            assert!(n < 7);
            VTag::new(format!("h{}", n))
        }
    }
}
