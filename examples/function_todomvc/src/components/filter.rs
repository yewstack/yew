use crate::state::Filter as FilterEnum;
use yew::{function_component, html, Callback, Properties};

#[derive(PartialEq, Properties)]
pub struct FilterProps {
    pub filter: FilterEnum,
    pub selected: bool,
    pub set_filter_onclick: Callback<FilterEnum>,
}

#[function_component(Filter)]
pub fn filter(props: &FilterProps) -> Html {
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
               onclick={props.set_filter_onclick.reform(move |_| filter)}
            >
                { props.filter }
            </a>
        </li>
    }
}
