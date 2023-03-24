use super::*;

/// The `Parent` component is the parent of the `Child` component. It has no logic, and is here to
/// show there is no direct relation between grandchild and grandparent.
#[function_component]
pub fn Parent() -> Html {
    html! {
        <div class="border-4 border-orange-500 rounded-2xl">
            <div class="bg-orange-500 rounded-t px-3 pb-1 font-medium">
                <span>{ "Parent" }</span>
            </div>
            <div class="flex gap-x-5 px-5 pb-5 pt-3">
                <Child />
            </div>
        </div>
    }
}
