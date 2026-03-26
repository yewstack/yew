use stylist::yew::styled_component;
use yew::prelude::*;

use crate::components::admonition::Admonition;
pub use crate::components::admonition::AdmonitionType;
use crate::components::code_block::CodeBlock;
use crate::components::tabs::{TabItem, Tabs};

#[derive(Clone, PartialEq, Properties)]
pub struct ContentLinkProps {
    pub href: AttrValue,
    pub children: Html,
}

#[styled_component]
pub fn ContentLink(props: &ContentLinkProps) -> Html {
    html! {
        <a class={css!(
            color: var(--color-primary);
            text-decoration: none;

            &:hover {
                text-decoration: underline;
            }
        )} href={props.href.clone()}>{props.children.clone()}</a>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct ContentTableProps {
    children: Html,
}

#[styled_component]
fn ContentTable(props: &ContentTableProps) -> Html {
    html! {
        <div class={css!(overflow-x: auto; margin-bottom: 1rem;)}>
            <table class={css!(
                width: 100%;
                border-collapse: collapse;
                min-width: 400px;
                & th, & td {
                    border: 1px solid var(--color-border);
                    padding: 0.5rem 0.75rem;
                    text-align: left;
                }
                & th {
                    background: var(--color-bg-secondary);
                    font-weight: 600;
                }
            )}>
                {props.children.clone()}
            </table>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct AnchorHeadingProps {
    level: u8,
    id: AttrValue,
    children: Html,
    hash_href: AttrValue,
    hash_label: AttrValue,
}

#[styled_component]
fn AnchorHeading(props: &AnchorHeadingProps) -> Html {
    let id = Some(props.id.clone());
    let content = html! { <>
        {props.children.clone()}
        <a class={css!(
            opacity: 0;
            transition: opacity 0.2s;
            padding-left: 0.5rem;
            color: var(--color-primary);
            text-decoration: none;
            font-weight: normal;
        )} href={props.hash_href.clone()} aria-label={props.hash_label.clone()}>{"#"}</a>
    </> };
    match props.level {
        2 => {
            html! { <h2 {id} class={css!("scroll-margin-top: calc(var(--navbar-height) + 1rem);")}>{content}</h2> }
        }
        3 => {
            html! { <h3 {id} class={css!("scroll-margin-top: calc(var(--navbar-height) + 1rem);")}>{content}</h3> }
        }
        _ => {
            html! { <h4 {id} class={css!("scroll-margin-top: calc(var(--navbar-height) + 1rem);")}>{content}</h4> }
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Inline {
    Text(AttrValue),
    Code(AttrValue),
    Bold(Vec<Inline>),
    Italic(Vec<Inline>),
    Link {
        href: AttrValue,
        children: Vec<Inline>,
    },
    LineBreak,
    Superscript(Vec<Inline>),
}

#[derive(Clone, PartialEq)]
pub enum ListItem {
    Inline(Vec<Inline>),
    Blocks(Vec<Block>),
}

#[derive(Clone, PartialEq)]
pub struct TabData {
    pub value: AttrValue,
    pub label: AttrValue,
    pub children: Vec<Block>,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum CodeAnnotation {
    #[default]
    Default,
    NoRun,
    Ignore,
    CompileFail,
}

#[derive(Clone, PartialEq)]
pub enum Block {
    Heading {
        level: u8,
        children: Vec<Inline>,
        id: Option<AttrValue>,
    },
    Paragraph(Vec<Inline>),
    CodeBlock {
        language: AttrValue,
        code: AttrValue,
        title: Option<AttrValue>,
        annotation: CodeAnnotation,
    },
    UnorderedList(Vec<ListItem>),
    OrderedList(Vec<ListItem>),
    Table {
        headers: Vec<Vec<Inline>>,
        rows: Vec<Vec<Vec<Inline>>>,
    },
    Blockquote(Vec<Block>),
    Admonition {
        kind: AdmonitionType,
        title: Option<AttrValue>,
        children: Vec<Block>,
    },
    Tabs {
        default_value: AttrValue,
        items: Vec<TabData>,
    },
    Image {
        src: AttrValue,
        alt: AttrValue,
    },
    ThemedImage {
        light_src: AttrValue,
        dark_src: AttrValue,
        alt: AttrValue,
    },
    HorizontalRule,
}

#[macro_export]
macro_rules! bold { ($($e:expr),* $(,)?) => { $crate::content::bold(vec![$($e.into()),*]) }; }
#[macro_export]
macro_rules! italic { ($($e:expr),* $(,)?) => { $crate::content::italic(vec![$($e.into()),*]) }; }
#[macro_export]
macro_rules! sup { ($($e:expr),* $(,)?) => { $crate::content::sup(vec![$($e.into()),*]) }; }
#[macro_export]
macro_rules! link { ($href:expr, $($e:expr),* $(,)?) => { $crate::content::link($href, vec![$($e.into()),*]) }; }
#[macro_export]
macro_rules! h1 { ($($e:expr),* $(,)?) => { $crate::content::h1(vec![$($e.into()),*]) }; }
#[macro_export]
macro_rules! h2 { ($($e:expr),* $(,)?) => { $crate::content::h2(vec![$($e.into()),*]) }; }
#[macro_export]
macro_rules! h3 { ($($e:expr),* $(,)?) => { $crate::content::h3(vec![$($e.into()),*]) }; }
#[macro_export]
macro_rules! h4 { ($($e:expr),* $(,)?) => { $crate::content::h4(vec![$($e.into()),*]) }; }
#[macro_export]
macro_rules! h5 { ($($e:expr),* $(,)?) => { $crate::content::h5(vec![$($e.into()),*]) }; }
#[macro_export]
macro_rules! h2_id { ($id:expr, $($e:expr),* $(,)?) => { $crate::content::h2_id($id, vec![$($e.into()),*]) }; }
#[macro_export]
macro_rules! h3_id { ($id:expr, $($e:expr),* $(,)?) => { $crate::content::h3_id($id, vec![$($e.into()),*]) }; }
#[macro_export]
macro_rules! h4_id { ($id:expr, $($e:expr),* $(,)?) => { $crate::content::h4_id($id, vec![$($e.into()),*]) }; }
#[macro_export]
macro_rules! p { ($($e:expr),* $(,)?) => { $crate::content::p(vec![$($e.into()),*]) }; }
#[macro_export]
macro_rules! li { ($($e:expr),* $(,)?) => { $crate::content::li(vec![$($e.into()),*]) }; }
#[macro_export]
macro_rules! li_blocks { ($($e:expr),* $(,)?) => { $crate::content::li_blocks(vec![$($e.into()),*]) }; }
#[macro_export]
macro_rules! ul { ($($e:expr),* $(,)?) => { $crate::content::ul(vec![$($e.into()),*]) }; }
#[macro_export]
macro_rules! ol { ($($e:expr),* $(,)?) => { $crate::content::ol(vec![$($e.into()),*]) }; }
#[macro_export]
macro_rules! blockquote { ($($e:expr),* $(,)?) => { $crate::content::blockquote(vec![$($e.into()),*]) }; }
#[macro_export]
macro_rules! admonition { ($kind:expr, $title:expr, $($e:expr),* $(,)?) => { $crate::content::admonition($kind, $title, vec![$($e.into()),*]) }; }
#[macro_export]
macro_rules! tabs { ($default:expr, $($e:expr),* $(,)?) => { $crate::content::tabs($default, vec![$($e.into()),*]) }; }
#[macro_export]
macro_rules! tab { ($value:expr, $label:expr, $($e:expr),* $(,)?) => { $crate::content::tab($value, $label, vec![$($e.into()),*]) }; }

#[doc(hidden)]
pub use crate::{
    admonition, blockquote, bold, h1, h2, h2_id, h3, h3_id, h4, h4_id, h5, italic, li, li_blocks,
    link, ol, p, sup, tab, tabs, ul,
};

impl From<&'static str> for Inline {
    fn from(s: &'static str) -> Self {
        Inline::Text(s.into())
    }
}

pub fn code(s: impl Into<AttrValue>) -> Inline {
    Inline::Code(s.into())
}
pub fn bold(children: Vec<Inline>) -> Inline {
    Inline::Bold(children)
}
pub fn italic(children: Vec<Inline>) -> Inline {
    Inline::Italic(children)
}
pub fn link(href: impl Into<AttrValue>, children: Vec<Inline>) -> Inline {
    Inline::Link {
        href: href.into(),
        children,
    }
}
pub fn br() -> Inline {
    Inline::LineBreak
}
pub fn sup(children: Vec<Inline>) -> Inline {
    Inline::Superscript(children)
}

pub fn h1(children: Vec<Inline>) -> Block {
    Block::Heading {
        level: 1,
        children,
        id: None,
    }
}
pub fn h2(children: Vec<Inline>) -> Block {
    Block::Heading {
        level: 2,
        children,
        id: None,
    }
}
pub fn h3(children: Vec<Inline>) -> Block {
    Block::Heading {
        level: 3,
        children,
        id: None,
    }
}
pub fn h4(children: Vec<Inline>) -> Block {
    Block::Heading {
        level: 4,
        children,
        id: None,
    }
}
pub fn h5(children: Vec<Inline>) -> Block {
    Block::Heading {
        level: 5,
        children,
        id: None,
    }
}
pub fn h2_id(id: impl Into<AttrValue>, children: Vec<Inline>) -> Block {
    Block::Heading {
        level: 2,
        children,
        id: Some(id.into()),
    }
}
pub fn h3_id(id: impl Into<AttrValue>, children: Vec<Inline>) -> Block {
    Block::Heading {
        level: 3,
        children,
        id: Some(id.into()),
    }
}
pub fn h4_id(id: impl Into<AttrValue>, children: Vec<Inline>) -> Block {
    Block::Heading {
        level: 4,
        children,
        id: Some(id.into()),
    }
}
pub fn p(children: Vec<Inline>) -> Block {
    Block::Paragraph(children)
}
pub fn code_block(language: impl Into<AttrValue>, code: impl Into<AttrValue>) -> Block {
    Block::CodeBlock {
        language: language.into(),
        code: code.into(),
        title: None,
        annotation: CodeAnnotation::Default,
    }
}
pub fn code_block_title(
    language: impl Into<AttrValue>,
    title: impl Into<AttrValue>,
    code: impl Into<AttrValue>,
) -> Block {
    Block::CodeBlock {
        language: language.into(),
        code: code.into(),
        title: Some(title.into()),
        annotation: CodeAnnotation::Default,
    }
}
pub fn code_block_no_run(language: impl Into<AttrValue>, code: impl Into<AttrValue>) -> Block {
    Block::CodeBlock {
        language: language.into(),
        code: code.into(),
        title: None,
        annotation: CodeAnnotation::NoRun,
    }
}
pub fn code_block_ignore(language: impl Into<AttrValue>, code: impl Into<AttrValue>) -> Block {
    Block::CodeBlock {
        language: language.into(),
        code: code.into(),
        title: None,
        annotation: CodeAnnotation::Ignore,
    }
}
pub fn code_block_compile_fail(
    language: impl Into<AttrValue>,
    code: impl Into<AttrValue>,
) -> Block {
    Block::CodeBlock {
        language: language.into(),
        code: code.into(),
        title: None,
        annotation: CodeAnnotation::CompileFail,
    }
}
pub fn code_block_title_no_run(
    language: impl Into<AttrValue>,
    title: impl Into<AttrValue>,
    code: impl Into<AttrValue>,
) -> Block {
    Block::CodeBlock {
        language: language.into(),
        code: code.into(),
        title: Some(title.into()),
        annotation: CodeAnnotation::NoRun,
    }
}
pub fn ul(items: Vec<ListItem>) -> Block {
    Block::UnorderedList(items)
}
pub fn ol(items: Vec<ListItem>) -> Block {
    Block::OrderedList(items)
}
pub fn li(inlines: Vec<Inline>) -> ListItem {
    ListItem::Inline(inlines)
}
pub fn li_blocks(blocks: Vec<Block>) -> ListItem {
    ListItem::Blocks(blocks)
}
pub fn table(headers: Vec<Vec<Inline>>, rows: Vec<Vec<Vec<Inline>>>) -> Block {
    Block::Table { headers, rows }
}
pub fn blockquote(children: Vec<Block>) -> Block {
    Block::Blockquote(children)
}
pub fn admonition(kind: AdmonitionType, title: Option<&str>, children: Vec<Block>) -> Block {
    Block::Admonition {
        kind,
        title: title.map(AttrValue::from),
        children,
    }
}
pub fn img(src: impl Into<AttrValue>, alt: impl Into<AttrValue>) -> Block {
    Block::Image {
        src: src.into(),
        alt: alt.into(),
    }
}
pub fn themed_img(
    light_src: impl Into<AttrValue>,
    dark_src: impl Into<AttrValue>,
    alt: impl Into<AttrValue>,
) -> Block {
    Block::ThemedImage {
        light_src: light_src.into(),
        dark_src: dark_src.into(),
        alt: alt.into(),
    }
}
pub fn tabs(default_value: impl Into<AttrValue>, items: Vec<TabData>) -> Block {
    Block::Tabs {
        default_value: default_value.into(),
        items,
    }
}
pub fn tab(
    value: impl Into<AttrValue>,
    label: impl Into<AttrValue>,
    children: Vec<Block>,
) -> TabData {
    TabData {
        value: value.into(),
        label: label.into(),
        children,
    }
}
pub fn hr() -> Block {
    Block::HorizontalRule
}

impl Inline {
    pub fn to_plain_text(&self) -> String {
        match self {
            Inline::Text(t) => t.to_string(),
            Inline::Code(c) => c.to_string(),
            Inline::Bold(children) | Inline::Italic(children) => {
                children.iter().map(Inline::to_plain_text).collect()
            }
            Inline::Link { children, .. } => children.iter().map(Inline::to_plain_text).collect(),
            Inline::LineBreak => String::new(),
            Inline::Superscript(children) => children.iter().map(Inline::to_plain_text).collect(),
        }
    }

    pub fn to_html(&self) -> Html {
        match self {
            Inline::Text(t) => html! { {t} },
            Inline::Code(c) => html! { <code>{c}</code> },
            Inline::Bold(children) => {
                html! { <strong>for child in children { {child.to_html()} }</strong> }
            }
            Inline::Italic(children) => {
                html! { <em>for child in children { {child.to_html()} }</em> }
            }
            Inline::Link { href, children } => html! {
                <ContentLink href={href.clone()}>for child in children { {child.to_html()} }</ContentLink>
            },
            Inline::LineBreak => html! { <br /> },
            Inline::Superscript(children) => {
                html! { <sup>for child in children { {child.to_html()} }</sup> }
            }
        }
    }

    pub fn to_markdown(&self) -> String {
        match self {
            Inline::Text(t) => t.to_string(),
            Inline::Code(c) => format!("`{c}`"),
            Inline::Bold(children) => {
                let inner: String = children.iter().map(Inline::to_markdown).collect();
                format!("**{inner}**")
            }
            Inline::Italic(children) => {
                let inner: String = children.iter().map(Inline::to_markdown).collect();
                format!("*{inner}*")
            }
            Inline::Link { href, children } => {
                let inner: String = children.iter().map(Inline::to_markdown).collect();
                format!("[{inner}]({href})")
            }
            Inline::LineBreak => "\n".to_string(),
            Inline::Superscript(children) => {
                let inner: String = children.iter().map(Inline::to_markdown).collect();
                format!("<sup>{inner}</sup>")
            }
        }
    }
}

fn strip_highlight_comments(code: &str) -> String {
    let mut out = String::new();
    for line in code.lines() {
        let trimmed = line.trim();
        if trimmed == "// highlight-next-line"
            || trimmed == "//highlight-next-line"
            || trimmed == "// highlight-start"
            || trimmed == "//highlight-start"
            || trimmed == "// highlight-end"
            || trimmed == "//highlight-end"
        {
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}

fn list_item_markdown(item: &ListItem, prefix: &str, indent: usize) -> String {
    let pad = "  ".repeat(indent);
    match item {
        ListItem::Inline(inlines) => {
            let inner: String = inlines.iter().map(Inline::to_markdown).collect();
            format!("{pad}{prefix}{}\n", inner.trim())
        }
        ListItem::Blocks(blocks) => {
            let mut out = String::new();
            for (i, block) in blocks.iter().enumerate() {
                match block {
                    Block::UnorderedList(items) => {
                        for sub in items {
                            out.push_str(&list_item_markdown(sub, "- ", indent + 1));
                        }
                    }
                    Block::OrderedList(items) => {
                        for (j, sub) in items.iter().enumerate() {
                            out.push_str(&list_item_markdown(
                                sub,
                                &format!("{}. ", j + 1),
                                indent + 1,
                            ));
                        }
                    }
                    Block::Paragraph(inlines) => {
                        let inner: String = inlines.iter().map(Inline::to_markdown).collect();
                        if i == 0 {
                            out.push_str(&format!("{pad}{prefix}{}\n", inner.trim()));
                        } else {
                            out.push_str(&format!(
                                "{pad}{}{}\n",
                                " ".repeat(prefix.len()),
                                inner.trim()
                            ));
                        }
                    }
                    _ => {
                        let md = block.to_markdown();
                        if i == 0 {
                            for (j, line) in md.trim().lines().enumerate() {
                                if j == 0 {
                                    out.push_str(&format!("{pad}{prefix}{line}\n"));
                                } else {
                                    out.push_str(&format!(
                                        "{pad}{}{line}\n",
                                        " ".repeat(prefix.len())
                                    ));
                                }
                            }
                        } else {
                            for line in md.trim().lines() {
                                out.push_str(&format!("{pad}{}{line}\n", " ".repeat(prefix.len())));
                            }
                        }
                    }
                }
            }
            out
        }
    }
}

impl Block {
    pub fn to_html(&self) -> Html {
        match self {
            Block::Heading {
                level,
                children,
                id,
            } => {
                let text: String = children.iter().map(Inline::to_plain_text).collect();
                let id = id
                    .clone()
                    .unwrap_or_else(|| AttrValue::from(slugify(&text)));
                if *level >= 2 && *level <= 4 && !id.is_empty() {
                    let href = format!("#{id}");
                    let label = format!("Direct link to {text}");
                    html! {
                        <AnchorHeading level={*level} id={id} hash_href={href} hash_label={label}>
                            for child in children { {child.to_html()} }
                        </AnchorHeading>
                    }
                } else {
                    let id = if id.is_empty() { None } else { Some(id) };
                    let content = html! { <>for child in children { {child.to_html()} }</> };
                    match level {
                        1 => html! { <h1 {id}>{content}</h1> },
                        2 => html! { <h2 {id}>{content}</h2> },
                        3 => html! { <h3 {id}>{content}</h3> },
                        4 => html! { <h4 {id}>{content}</h4> },
                        5 => html! { <h5 {id}>{content}</h5> },
                        _ => html! { <h6 {id}>{content}</h6> },
                    }
                }
            }
            Block::Paragraph(children) => html! {
                <p>for child in children { {child.to_html()} }</p>
            },
            Block::CodeBlock {
                language,
                code,
                title,
                ..
            } => {
                let title = title.clone().unwrap_or_default();
                html! {
                    <CodeBlock language={language.clone()} code={code.clone()} title={title} />
                }
            }
            Block::UnorderedList(items) => html! {
                <ul>
                    for item in items { {match item {
                        ListItem::Inline(inlines) => html! {
                            <li>for inline in inlines { {inline.to_html()} }</li>
                        },
                        ListItem::Blocks(blocks) => html! {
                            <li>for block in blocks { {block.to_html()} }</li>
                        },
                    }} }
                </ul>
            },
            Block::OrderedList(items) => html! {
                <ol>
                    for item in items { {match item {
                        ListItem::Inline(inlines) => html! {
                            <li>for inline in inlines { {inline.to_html()} }</li>
                        },
                        ListItem::Blocks(blocks) => html! {
                            <li>for block in blocks { {block.to_html()} }</li>
                        },
                    }} }
                </ol>
            },
            Block::Table { headers, rows } => {
                let header_cells: Vec<Html> = headers
                    .iter()
                    .map(|h| html! { <th>{ for h.iter().map(Inline::to_html) }</th> })
                    .collect();
                let body_rows: Vec<Html> = rows
                    .iter()
                    .map(|row| {
                        html! {
                            <tr>
                                { for row.iter().map(|cell| html! {
                                    <td>{ for cell.iter().map(Inline::to_html) }</td>
                                })}
                            </tr>
                        }
                    })
                    .collect();
                html! {
                    <ContentTable>
                        <thead><tr>{ for header_cells.into_iter() }</tr></thead>
                        <tbody>{ for body_rows.into_iter() }</tbody>
                    </ContentTable>
                }
            }
            Block::Blockquote(children) => html! {
                <blockquote>for child in children { {child.to_html()} }</blockquote>
            },
            Block::Admonition {
                kind,
                title,
                children,
            } => {
                let title_str = title.clone().unwrap_or_default();
                html! {
                    <Admonition kind={*kind} title={title_str}>
                        for child in children { {child.to_html()} }
                    </Admonition>
                }
            }
            Block::Tabs {
                default_value,
                items,
            } => {
                let tab_children = items.iter().map(|t| {
                    yew::html_nested! {
                        <TabItem value={t.value.clone()} label={t.label.clone()}>
                            for child in t.children.iter() { {child.to_html()} }
                        </TabItem>
                    }
                });
                html! {
                    <Tabs default_value={default_value.clone()}>
                        { for tab_children }
                    </Tabs>
                }
            }
            Block::Image { src, alt } => html! {
                <img src={src.clone()} alt={alt.clone()} />
            },
            Block::ThemedImage {
                light_src,
                dark_src,
                alt,
            } => html! {
                <>
                    <img src={light_src.clone()} alt={alt.clone()} class="themed-img-light" />
                    <img src={dark_src.clone()} alt={alt.clone()} class="themed-img-dark" />
                </>
            },
            Block::HorizontalRule => html! { <hr /> },
        }
    }

    pub fn to_markdown(&self) -> String {
        match self {
            Block::Heading {
                level, children, ..
            } => {
                let prefix = "#".repeat(*level as usize);
                let inner: String = children.iter().map(Inline::to_markdown).collect();
                format!("{prefix} {inner}\n\n")
            }
            Block::Paragraph(children) => {
                let inner: String = children.iter().map(Inline::to_markdown).collect();
                format!("{inner}\n\n")
            }
            Block::CodeBlock {
                language,
                code,
                title,
                annotation,
            } => {
                let cleaned = strip_highlight_comments(code);
                let annotation_str = match annotation {
                    CodeAnnotation::Default => String::new(),
                    CodeAnnotation::NoRun => " ,no_run".to_string(),
                    CodeAnnotation::Ignore => " ,ignore".to_string(),
                    CodeAnnotation::CompileFail => " ,compile_fail".to_string(),
                };
                let title_suffix = title
                    .as_ref()
                    .map_or(String::new(), |t| format!(" title=\"{t}\""));
                format!(
                    "```{language}{annotation_str}{title_suffix}\n{}```\n\n",
                    cleaned.trim_end_matches('\n')
                )
            }
            Block::UnorderedList(items) => {
                let mut out = String::new();
                for item in items {
                    out.push_str(&list_item_markdown(item, "- ", 0));
                }
                out.push('\n');
                out
            }
            Block::OrderedList(items) => {
                let mut out = String::new();
                for (i, item) in items.iter().enumerate() {
                    out.push_str(&list_item_markdown(item, &format!("{}. ", i + 1), 0));
                }
                out.push('\n');
                out
            }
            Block::Table { headers, rows } => {
                let mut out = String::from("| ");
                for h in headers {
                    let inner: String = h.iter().map(Inline::to_markdown).collect();
                    out.push_str(&format!("{inner} | "));
                }
                out.push('\n');
                out.push_str("| ");
                for _ in headers {
                    out.push_str("--- | ");
                }
                out.push('\n');
                for row in rows {
                    out.push_str("| ");
                    for cell in row {
                        let inner: String = cell.iter().map(Inline::to_markdown).collect();
                        out.push_str(&format!("{inner} | "));
                    }
                    out.push('\n');
                }
                out.push('\n');
                out
            }
            Block::Blockquote(children) => {
                let content: String = children.iter().map(Block::to_markdown).collect();
                let mut out = String::new();
                for line in content.trim().lines() {
                    out.push_str(&format!("> {line}\n"));
                }
                out.push('\n');
                out
            }
            Block::Admonition {
                kind,
                title,
                children,
            } => {
                let display = title
                    .as_ref()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| capitalize(kind.label()));
                let content: String = children.iter().map(Block::to_markdown).collect();
                let mut out = format!("> **{display}**\n");
                for line in content.trim().lines() {
                    out.push_str(&format!("> {line}\n"));
                }
                out.push('\n');
                out
            }
            Block::Tabs { items, .. } => {
                let mut out = String::new();
                for t in items {
                    out.push_str(&format!("**{}:**\n\n", t.label));
                    for child in &t.children {
                        out.push_str(&child.to_markdown());
                    }
                }
                out
            }
            Block::Image { src, alt } => format!("![{alt}]({src})\n\n"),
            Block::ThemedImage { light_src, alt, .. } => format!("![{alt}]({light_src})\n\n"),
            Block::HorizontalRule => "---\n\n".to_string(),
        }
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(ch) => format!("{}{}", ch.to_uppercase(), c.as_str()),
        None => String::new(),
    }
}

fn slugify(text: &str) -> String {
    let mut slug = String::with_capacity(text.len());
    let mut prev_dash = true;
    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            prev_dash = false;
        } else if !prev_dash {
            slug.push('-');
            prev_dash = true;
        }
    }
    slug.trim_end_matches('-').to_string()
}

#[derive(Clone, PartialEq)]
pub struct TocEntry {
    pub id: AttrValue,
    pub text: AttrValue,
    pub level: u8,
}

pub struct Content {
    pub blocks: Vec<Block>,
}

impl Content {
    pub fn new(blocks: Vec<Block>) -> Self {
        Self { blocks }
    }

    pub fn toc_entries(&self) -> Vec<TocEntry> {
        self.blocks
            .iter()
            .filter_map(|block| match block {
                Block::Heading {
                    level,
                    children,
                    id,
                } if *level >= 2 && *level <= 4 => {
                    let text: String = children.iter().map(Inline::to_plain_text).collect();
                    let id = id
                        .clone()
                        .unwrap_or_else(|| AttrValue::from(slugify(&text)));
                    if id.is_empty() {
                        return None;
                    }
                    Some(TocEntry {
                        id,
                        text: AttrValue::from(text),
                        level: *level,
                    })
                }
                _ => None,
            })
            .collect()
    }

    pub fn to_html(&self) -> Html {
        html! { <>for block in &self.blocks { {block.to_html()} }</> }
    }

    pub fn to_markdown(&self) -> String {
        self.blocks
            .iter()
            .map(Block::to_markdown)
            .collect::<String>()
            .trim()
            .to_string()
    }
}
