/// Original author of this code is [Nathan Ringo](https://github.com/remexre)
/// Source: https://github.com/acmumn/mentoring/blob/master/web-client/src/view/markdown.rs

use pulldown_cmark::{Alignment, Event, OPTION_ENABLE_TABLES, Parser, Tag};
use yew::html::Html;
use yew::virtual_dom::{VNode, VTag, VText};

pub fn render<T>(source: &str) -> Html<T> {
    let mut stack: Vec<VTag<T>> = Vec::new();
    let mut active_tag: VTag<T> = VTag::new("markdown");
    for ev in Parser::new_ext(source, OPTION_ENABLE_TABLES) {
        match ev {
            Event::Start(tag) => {
                let mut vtag = make_tag(tag);
                ::std::mem::swap(&mut vtag, &mut active_tag);
                stack.push(vtag.into());
            }
            Event::End(tag) => {
                let mut vtag = stack.pop().unwrap();
                ::std::mem::swap(&mut vtag, &mut active_tag);
                active_tag.add_child(vtag.into());
            }
            Event::Text(text) => {
                let node = VText::new(text).into();
                active_tag.add_child(node);
            }
            Event::SoftBreak => {
                let node = VText::new("\n").into();
                active_tag.add_child(node);
            },
            Event::HardBreak => {
                let node = VTag::new("br").into();
                active_tag.add_child(node);
            },
            _ => println!("Unsupported event: {:#?}", ev),
        }
    }
    active_tag
}

fn make_tag<M>(t: Tag) -> VTag<M> {
    match t {
        Tag::Paragraph => VTag::new("p"),
        Tag::Rule => VTag::new("hr"),
        Tag::Header(n) => {
            assert!(n > 0);
            assert!(n < 7);
            VTag::new(format!("h{}", n))
        }
        Tag::BlockQuote => {
            let mut el = VTag::new("blockquote");
            el.add_classes("blockquote");
            el
        }
        Tag::CodeBlock(lang) => {
            let mut el = VTag::new("code");
            el
        }
        Tag::List(None) => VTag::new("ul"),
        Tag::List(Some(1)) => VTag::new("ol"),
        Tag::List(Some(start)) => {
            let mut el = VTag::new("ol");
            el.add_attribute("start", start);
            el
        }
        Tag::Item => VTag::new("li"),
        Tag::Table(_) => {
            let mut el = VTag::new("table");
            el.add_classes("table");
            el
        }
        Tag::TableHead => VTag::new("tr"),
        Tag::TableRow => VTag::new("tr"),
        Tag::TableCell => VTag::new("td"),
        Tag::Emphasis => {
            let mut el = VTag::new("span");
            el.add_classes("font-italic");
            el
        }
        Tag::Strong => {
            let mut el = VTag::new("span");
            el.add_classes("font-weight-bold");
            el
        }
        Tag::Code => VTag::new("code"),
        Tag::Link(href, title) => {
            let mut el = VTag::new("a");
            el.add_attribute("href", href);
            if title != "" {
                el.add_attribute("title", title);
            }
            el
        }
        Tag::Image(src, title) => {
            let mut el = VTag::new("img");
            el.add_attribute("src", src);
            if title != "" {
                el.add_attribute("title", title);
            }
            el
        }
        _ => unimplemented!("tag {:?}", t)
    }
}
