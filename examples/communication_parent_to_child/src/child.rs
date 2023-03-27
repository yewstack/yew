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
            <div class="child-body">
                <div class="child-tag">
                    <span>{ "Child" }</span>
                </div>
                <div class="child-content">
                    <span>{ "My parent has been clicked " }<span>{ ctx.props().clicks }</span>{ " times." }</span>
                </div>
            </div>
        }
    }
}
