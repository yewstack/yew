use super::*;

/// The `Parent` component is the parent of the `Child` component. It has no logic, and is here to
/// show there is no direct relation between grandchild and grandparent.
#[function_component]
pub fn Parent() -> Html {
    html! {
        <Child />
    }
}