// Naming this file use_context could be confusing. Not least to the IDE.
use super::get_current_scope;
use std::any::TypeId;
use std::iter;
use yew::html::{AnyScope, Renderable};
use yew::{html, Children, Component, ComponentLink, Html, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct ContextProviderProps<T: Clone> {
    pub context: T,
    pub children: Children,
}

pub struct ContextProvider<T: Clone + 'static> {
    props: ContextProviderProps<T>,
}

impl<T: Clone + 'static> Component for ContextProvider<T> {
    type Message = ();
    type Properties = ContextProviderProps<T>;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        ContextProvider { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true // TODO
    }

    fn view(&self) -> Html {
        return html! {
            <>
                { self.props.children.render() }
            </>
        };
    }
}

pub fn use_context<T: 'static + Clone>() -> Option<T> {
    let scope = get_current_scope()
        .expect("No current Scope. `use_context` can only be called inside functional components");

    let expected_type_id = TypeId::of::<ContextProvider<T>>();
    iter::successors(Some(&scope), |scope| scope.get_parent())
        .filter(|scope| scope.get_type_id() == &expected_type_id)
        .cloned()
        .map(AnyScope::downcast::<ContextProvider<T>>)
        .flat_map(|scope| scope.get_component().map(|comp| comp.props.context.clone()))
        .next()
}
