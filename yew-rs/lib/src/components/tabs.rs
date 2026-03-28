use yew::prelude::*;
use yew_site_proc::comp;

#[comp]
pub fn Tabs(children: ChildrenWithProps<TabItem>, #[prop_or_default] default_value: AttrValue) {
    let active = use_state(|| {
        if default_value.is_empty() {
            children
                .iter()
                .next()
                .map(|c| c.props.value.to_string())
                .unwrap_or_default()
        } else {
            default_value.to_string()
        }
    });

    html! {
        <div class={css!(margin-bottom: 1.5rem;)}>
            <ul class={css!(display: flex; list-style: none; padding: 0; margin: 0; border-bottom: 2px solid var(--color-border); gap: 0;)} role="tablist">
                for child in children.iter(){
                    <li
                        class={{
                            let value = child.props.value.clone();
                            let is_active = *active == value.as_str();
                            css!(padding: 0.75rem 1.25rem; cursor: pointer; font-weight: 500; font-size: 0.875rem; color: ${if is_active { "var(--color-primary)" } else { "var(--color-text-secondary)" }}; border-bottom: 2px solid ${if is_active { "var(--color-primary)" } else { "transparent" }}; margin-bottom: -2px; transition: color 0.2s, border-color 0.2s; user-select: none; &:hover { color: var(--color-primary); }) }}
                        role="tab"
                        onclick={let active = active.clone(); let value = child.props.value.clone(); move |_: MouseEvent| active.set(value.to_string())}
                    >
                        {child.props.label.clone()}
                    </li>

                }
            </ul>
            for child in children.iter(){
                <div class={css!(display: ${if *active == child.props.value.as_str() { "block" } else { "none" }}; padding: 1rem 0;)} role="tabpanel">
                    {child.props.children.clone()}
                </div>
            }
        </div>
    }
}

#[comp]
pub fn TabItem(value: AttrValue, label: AttrValue, #[prop_or_default] children: Html) {
    html! { {children} }
}
