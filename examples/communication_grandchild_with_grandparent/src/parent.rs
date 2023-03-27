use super::*;

/// The `Parent` component is the parent of the `Child` component. It has no logic, and is here to
/// show there is no direct relation between grandchild and grandparent.
#[function_component]
pub fn Parent() -> Html {
    html! {
        <div class="parent-body">
            <div class="parent-tag">
                <span>{ "Parent" }</span>
            </div>
            <div class="parent-content">
                <Child name="Alice" />
                <Child name="Bob" />
            </div>
        </div>
    }
}
