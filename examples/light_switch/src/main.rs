use yew::html::{BindableRef, Scope};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
struct SwitchProps {
    on_toggle: Callback<MouseEvent>,
}

#[function_component]
fn Switch(SwitchProps { on_toggle }: &SwitchProps) -> Html {
    html! {
        <div class="switchlabel">
            <label class="switch">
                <input type="checkbox" onclick={on_toggle} />
                <span class="drive">
                    <span class="buffer" />
                    <span class="pin" />
                </span>
            </label>
            {"Light switch"}
        </div>
    }
}

enum LightMessage {
    Toggle,
}

struct Light {
    is_on: bool,
}

impl ComponentWithRef for Light {
    type Message = LightMessage;
    type Properties = ();
    type Reference = Scope<Self>;

    fn create(ctx: &Context<Self>, bindable_ref: BindableRef<'_, Self::Reference>) -> Self {
        bindable_ref.bind(ctx.link().clone());
        Light { is_on: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, LightMessage::Toggle: LightMessage) -> bool {
        self.is_on = !self.is_on;
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!["light", self.is_on.then(|| "on")]} />
        }
    }
}

#[function_component]
fn Room() -> Html {
    let light_ref: ComponentRef<Light> = use_html_ref();
    let on_toggle = {
        let light_ref = light_ref.clone();
        Callback::from(move |_| {
            light_ref
                .get()
                .expect("a light to have rendered")
                .send_message(LightMessage::Toggle)
        })
    };
    html! {
        <>
            <Light ref={&light_ref} />
            <Switch {on_toggle} />
        </>
    }
}

fn main() {
    yew::Renderer::<Room>::new().render();
}
