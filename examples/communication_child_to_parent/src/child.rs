use super::*;

/// The `Child` component is the child of the `Parent` component, and will send updates to the
/// parent using a Callback.
pub struct Child;

#[derive(Clone, PartialEq, Properties)]
pub struct ChildProps {
    pub name: AttrValue,
    pub on_clicked: Callback<AttrValue>,
}

impl Component for Child {
    type Message = ();
    type Properties = ChildProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let name = format!("I'm {}.", ctx.props().name);
        let my_name = ctx.props().name.clone();

        // Here we emit the callback to the parent component, whenever the button is clicked.
        let onclick = ctx.props().on_clicked.reform(move |_| my_name.clone());

        html! {
            <div class="child-body">
                <div class="child-tag">
                    <span>{ "Child" }</span>
                </div>
                <div class="child-content">
                    <span>{ name }</span>
                    <button {onclick}>{"Click"}</button>
                </div>
            </div>
        }
    }
}
