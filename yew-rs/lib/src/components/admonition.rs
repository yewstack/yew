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
    fn css_class(self) -> &'static str {
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

    let style = css!(
        r#"
        margin-bottom: 1.5rem;
        padding: 1rem 1.25rem;
        border-radius: 8px;
        border-left: 4px solid;

        &.note {
            background: var(--admonition-note-bg);
            border-color: #636770;
        }

        &.tip {
            background: var(--admonition-tip-bg);
            border-color: #00a400;
        }

        &.info {
            background: var(--admonition-info-bg);
            border-color: #4c8dff;
        }

        &.warning {
            background: var(--admonition-warning-bg);
            border-color: #e6a700;
        }

        &.danger {
            background: var(--admonition-danger-bg);
            border-color: #e13238;
        }

        &.caution {
            background: var(--admonition-caution-bg);
            border-color: #e6a700;
        }

        &.important {
            background: var(--admonition-important-bg);
            border-color: #a855f7;
        }

        .heading {
            font-weight: 700;
            text-transform: uppercase;
            font-size: 0.8125rem;
            letter-spacing: 0.05em;
            margin-bottom: 0.5rem;
        }

        .content p:last-child {
            margin-bottom: 0;
        }
    "#
    );

    html! {
        <div class={classes!(style, props.kind.css_class())}>
            <div class="heading">
                <span>{title}</span>
            </div>
            <div class="content">
                {props.children.clone()}
            </div>
        </div>
    }
}
