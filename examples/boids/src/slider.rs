use std::cell::Cell;
use yew::{html, Callback, Component, ComponentLink, Html, InputData, Properties, ShouldRender};

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
    props: Props,
    id: usize,
}
impl Component for Slider {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            props,
            id: next_slider_id(),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let Props {
            label,
            value,
            ref onchange,
            precision,
            percentage,
            min,
            max,
            step,
        } = self.props;

        let precision = precision.unwrap_or_else(|| if percentage { 1 } else { 0 });

        let display_value = if percentage {
            format!("{:.p$}%", 100.0 * value, p = precision)
        } else {
            format!("{:.p$}", value, p = precision)
        };

        let id = format!("slider-{}", self.id);
        let step = step.unwrap_or_else(|| {
            let p = if percentage { precision + 2 } else { precision };
            10f64.powi(-(p as i32))
        });

        html! {
            <div class="slider">
                <label for={id.clone()} class="slider__label">{ label }</label>
                <input type="range"
                    id={id}
                    class="slider__input"
                    min={min.to_string()} max={max.to_string()} step={step.to_string()}
                    oninput={onchange.reform(|data: InputData| data.value.parse().unwrap())}
                    value={value.to_string()}
                />
                <span class="slider__value">{ display_value }</span>
            </div>
        }
    }
}
