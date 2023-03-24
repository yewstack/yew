use super::*;

/// The `Child` component is the child of the `Parent` component, and will receive updates from the
/// parent using properties.
pub struct Child;

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct ChildProps {
    pub clicks: u32,
}

impl Component for Child {
    type Message = ();
    type Properties = ChildProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="border-4 border-orange-500 rounded-2xl flex-grow">
                <div class="bg-orange-500 rounded-t px-3 pb-1 font-medium">
                    <span>{ "Child" }</span>
                </div>
                <div class="px-5 py-3">
                    <span class="text-xl">{ "My parent has been clicked " }<span class="font-bold">{ ctx.props().clicks }</span>{ " times." }</span>
                </div>
            </div>
        }
    }
}
