use std::rc::Rc;

use gloo::timers::callback::Interval;
use instant::Instant;
use yew::prelude::*;

const RESOLUTION: u32 = 500;
const MIN_INTERVAL_MS: u32 = 50;

pub enum ValueAction {
    Tick,
    Props(Props),
}

#[derive(Clone, PartialEq, Debug)]
pub struct ValueState {
    start: Instant,

    value: f64,

    props: Props,
}

impl Reducible for ValueState {
    type Action = ValueAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Self::Action::Props(props) => Self {
                start: self.start,
                value: self.value,
                props,
            }
            .into(),

            Self::Action::Tick => {
                let elapsed = self.start.elapsed().as_millis() as u32;
                let value = elapsed as f64 / self.props.duration_ms as f64;

                let mut start = self.start;

                if elapsed > self.props.duration_ms {
                    self.props.on_complete.emit(());
                    start = Instant::now();
                } else {
                    self.props.on_progress.emit(self.value);
                }

                Self {
                    start,
                    value,
                    props: self.props.clone(),
                }
                .into()
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub duration_ms: u32,
    pub on_complete: Callback<()>,
    #[prop_or_default]
    pub on_progress: Callback<f64>,
}

#[function_component]
pub fn ProgressDelay(props: &Props) -> Html {
    let Props { duration_ms, .. } = props.clone();

    let value = {
        let props = props.clone();
        use_reducer(move || ValueState {
            start: Instant::now(),
            value: 0.0,

            props,
        })
    };

    {
        let value = value.clone();
        use_effect_with((), move |_| {
            let interval = (duration_ms / RESOLUTION).min(MIN_INTERVAL_MS);
            let interval = Interval::new(interval, move || value.dispatch(ValueAction::Tick));

            || {
                let _interval = interval;
            }
        });
    }

    {
        let value = value.clone();
        use_effect_with(props.clone(), move |props| {
            value.dispatch(ValueAction::Props(props.clone()));
            || {}
        });
    }

    let value = &value.value;

    html! {
        <progress class="progress is-primary" value={value.to_string()} max=1.0>
            { format!("{:.0}%", 100.0 * value) }
        </progress>
    }
}
