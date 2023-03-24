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
        let msg = format!("My parent has been clicked {} times", ctx.props().clicks);
        html! {
            <div class="child">
                <div>{msg}</div>
            </div>
        }
    }
}