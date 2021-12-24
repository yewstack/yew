use crate::state::Filter as FilterEnum;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct FilterProps {
    pub filter: FilterEnum,
    pub selected: bool,
    pub onset_filter: Callback<FilterEnum>,
}

#[function_component]
pub fn Filter(props: &FilterProps) -> Html {
    let filter = props.filter;

    let cls = if props.selected {
        "selected"
    } else {
        "not-selected"
    };

    html! {
        <li>
            <a class={cls}
               href={props.filter.as_href()}
               onclick={props.onset_filter.reform(move |_| filter)}
            >
                { props.filter }
            </a>
        </li>
    }
}
