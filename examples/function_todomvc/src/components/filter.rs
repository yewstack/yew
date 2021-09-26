use crate::state::Filter as FilterEnum;
use yew::{function_component, html, Callback, Properties};

#[derive(PartialEq, Properties, Clone)]
pub struct FilterProps {
    pub filter: FilterEnum,
    pub current_filter: FilterEnum,
    pub set_filter_onclick: Callback<FilterEnum>,
}

#[function_component(Filter)]
pub fn filter(props: &FilterProps) -> Html {
    let cls = if props.current_filter == props.filter {
        "selected"
    } else {
        "not-selected"
    };
    html! {
        <li>
            <a class={cls}
               href={props.filter.as_href()}
               onclick={
                   let props = props.clone();
                   move |_| {
                       props.set_filter_onclick.emit(props.filter)
                   }
                }
            >
                { props.filter }
            </a>
        </li>
    }
}
