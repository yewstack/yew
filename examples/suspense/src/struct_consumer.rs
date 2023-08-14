use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use crate::use_sleep;

#[function_component]
pub fn WithSleep<Comp>() -> HtmlResult
where
    Comp: BaseComponent<Properties = AppContentProps>,
{
    let sleep = use_sleep()?;
    let sleep = Callback::from(move |_| sleep());
    Ok(yew::virtual_dom::VChild::<Comp>::new(AppContentProps { resleep: sleep }, None).into())
}

#[derive(Debug, PartialEq, Properties)]
pub struct AppContentProps {
    pub resleep: Callback<()>,
}

pub type AppContent = WithSleep<BaseAppContent>;

pub enum Msg {
    ValueUpdate(String),
    TakeABreak,
}

pub struct BaseAppContent {
    value: String,
}

impl Component for BaseAppContent {
    type Message = Msg;
    type Properties = AppContentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: "I am writing a long story...".to_string(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ValueUpdate(v) => {
                self.value = v;
            }
            Msg::TakeABreak => {
                ctx.props().resleep.emit(());
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = ctx.link().callback(|e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into::<HtmlTextAreaElement>();
            Msg::ValueUpdate(input.value())
        });
        let on_take_a_break = ctx.link().callback(|_| Msg::TakeABreak);
        html! {
            <div class="content-area">
                <textarea value={self.value.clone()} {oninput}></textarea>
                <div class="action-area">
                    <button onclick={on_take_a_break}>{"Take a break!"}</button>
                    <div class="hint">{"You can take a break at anytime"}<br />{"and your work will be preserved."}</div>
                </div>
            </div>
        }
    }
}
