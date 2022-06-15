use yew::prelude::*;

use crate::state::Filter as FilterEnum;

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

    let onset_filter = {
        let onset_filter = props.onset_filter.clone();
        move |_| onset_filter.emit(filter)
    };

    html! {
        <li>
            <a class={cls}
               href={props.filter.as_href()}
               onclick={onset_filter}
            >
                { props.filter }
            </a>
        </li>
    }
}
