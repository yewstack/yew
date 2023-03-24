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
            <div class="border-4 border-orange-500 rounded-2xl flex-grow">
                <div class="bg-orange-500 rounded-t px-3 pb-1 font-medium">
                    <span>{ "Child" }</span>
                </div>
                <div class="flex px-5 py-5 justify-between items-center">
                    <span class="text-xl">{ name }</span>
                    <button class="bg-orange-500 hover:bg-orange-700 rounded-2xl text-lg pt-1 pb-2 px-3 font-medium" {onclick}>{"Click"}</button>
                </div>
            </div>
        }
    }
}