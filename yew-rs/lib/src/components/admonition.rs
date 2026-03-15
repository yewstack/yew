use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum AdmonitionType {
    Note,
    Tip,
    Info,
    Warning,
    Danger,
    Caution,
    Important,
}

impl AdmonitionType {
    pub fn label(self) -> &'static str {
        match self {
            Self::Note => "note",
            Self::Tip => "tip",
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Danger => "danger",
            Self::Caution => "caution",
            Self::Important => "important",
        }
    }

    fn bg(self) -> &'static str {
        match self {
            Self::Note => "var(--admonition-note-bg)",
            Self::Tip => "var(--admonition-tip-bg)",
            Self::Info => "var(--admonition-info-bg)",
            Self::Warning => "var(--admonition-warning-bg)",
            Self::Danger => "var(--admonition-danger-bg)",
            Self::Caution => "var(--admonition-caution-bg)",
            Self::Important => "var(--admonition-important-bg)",
        }
    }

    fn border(self) -> &'static str {
        match self {
            Self::Note => "#636770",
            Self::Tip => "#00a400",
            Self::Info => "#4c8dff",
            Self::Warning | Self::Caution => "#e6a700",
            Self::Danger => "#e13238",
            Self::Important => "#a855f7",
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct AdmonitionProps {
    pub children: Html,
    #[prop_or(AdmonitionType::Note)]
    pub kind: AdmonitionType,
    #[prop_or_default]
    pub title: AttrValue,
}

#[styled_component]
pub fn Admonition(props: &AdmonitionProps) -> Html {
    let title = if props.title.is_empty() {
        props.kind.label().to_string()
    } else {
        props.title.to_string()
    };

    let bg = props.kind.bg();
    let border = props.kind.border();

    html! {
        <div class={css!(
            margin-bottom: 1.5rem;
            padding: 1rem 1.25rem;
            border-radius: 8px;
            border-left: 4px solid ${border};
            background: ${bg};
        )}>
            <div class={css!(r#"
                font-weight: 700;
                text-transform: uppercase;
                font-size: 0.8125rem;
                letter-spacing: 0.05em;
                margin-bottom: 0.5rem;
            "#)}>
                <span>{title}</span>
            </div>
            <div class={css!("p:last-child { margin-bottom: 0; }")}>
                {props.children.clone()}
            </div>
        </div>
    }
}
