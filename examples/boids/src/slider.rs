use std::cell::Cell;

use web_sys::HtmlInputElement;
use yew::events::InputEvent;
use yew::{html, Callback, Component, Context, Html, Properties, TargetCast};

thread_local! {
    static SLIDER_ID: Cell<usize> = Cell::default();
}
fn next_slider_id() -> usize {
    SLIDER_ID.with(|cell| cell.replace(cell.get() + 1))
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub label: &'static str,
    pub value: f64,
    pub onchange: Callback<f64>,
    #[prop_or_default]
    pub precision: Option<usize>,
    #[prop_or_default]
    pub percentage: bool,
    #[prop_or_default]
    pub min: f64,
    pub max: f64,
    #[prop_or_default]
    pub step: Option<f64>,
}

pub struct Slider {
    id: usize,
}
impl Component for Slider {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            id: next_slider_id(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        unimplemented!()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            label,
            value,
            ref onchange,
            precision,
            percentage,
            min,
            max,
            step,
        } = *ctx.props();

        let precision = precision.unwrap_or_else(|| usize::from(percentage));

        let display_value = if percentage {
            format!("{:.p$}%", 100.0 * value, p = precision)
        } else {
            format!("{value:.precision$}")
        };

        let id = format!("slider-{}", self.id);
        let step = step.unwrap_or_else(|| {
            let p = if percentage { precision + 2 } else { precision };
            10f64.powi(-(p as i32))
        });

        let oninput = onchange.reform(|e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            input.value_as_number()
        });

        html! {
            <div class="slider">
                <label for={id.clone()} class="slider__label">{ label }</label>
                <input type="range"
                    value={value.to_string()}
                    {id}
                    class="slider__input"
                    min={min.to_string()} max={max.to_string()} step={step.to_string()}
                    {oninput}
                />
                <span class="slider__value">{ display_value }</span>
            </div>
        }
    }
}
