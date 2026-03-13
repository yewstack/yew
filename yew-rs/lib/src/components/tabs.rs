use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct TabsProps {
    pub children: ChildrenWithProps<TabItem>,
    #[prop_or_default]
    pub default_value: AttrValue,
}

#[styled_component]
pub fn Tabs(props: &TabsProps) -> Html {
    let active = use_state(|| {
        if props.default_value.is_empty() {
            props
                .children
                .iter()
                .next()
                .map(|c| c.props.value.to_string())
                .unwrap_or_default()
        } else {
            props.default_value.to_string()
        }
    });

    let style = css!(
        r#"
        margin-bottom: 1.5rem;

        .tabs {
            display: flex;
            list-style: none;
            padding: 0;
            margin: 0;
            border-bottom: 2px solid var(--color-border);
            gap: 0;
        }

        .item {
            padding: 0.75rem 1.25rem;
            cursor: pointer;
            font-weight: 500;
            font-size: 0.875rem;
            color: var(--color-text-secondary);
            border-bottom: 2px solid transparent;
            margin-bottom: -2px;
            transition: color 0.2s, border-color 0.2s;
            user-select: none;
        }

        .item:hover {
            color: var(--color-primary);
        }

        .item--active {
            color: var(--color-primary);
            border-bottom-color: var(--color-primary);
        }

        .panel--hidden {
            display: none;
        }

        .panel {
            padding: 1rem 0;
        }
    "#
    );

    html! {
        <div class={style}>
            <ul class="tabs" role="tablist">
                { for props.children.iter().map(|child| {
                    let value = child.props.value.clone();
                    let is_active = *active == value.as_str();
                    let onclick = {
                        let active = active.clone();
                        let value = value.clone();
                        Callback::from(move |_: MouseEvent| {
                            active.set(value.to_string());
                        })
                    };
                    html! {
                        <li
                            class={classes!("item", is_active.then_some("item--active"))}
                            role="tab"
                            {onclick}
                        >
                            {&child.props.label}
                        </li>
                    }
                })}
            </ul>
            { for props.children.iter().map(|child| {
                let is_active = *active == child.props.value.as_str();
                html! {
                    <div class={classes!("panel", (!is_active).then_some("panel--hidden"))} role="tabpanel">
                        {child.props.children.clone()}
                    </div>
                }
            })}
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct TabItemProps {
    pub value: AttrValue,
    pub label: AttrValue,
    #[prop_or_default]
    pub children: Html,
}

#[component]
pub fn TabItem(props: &TabItemProps) -> Html {
    html! { {props.children.clone()} }
}
