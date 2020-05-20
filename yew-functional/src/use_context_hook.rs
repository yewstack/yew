// Naming this file use_context could be confusing. Not least to the IDE.
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::DerefMut;
use yew::html::Renderable;
use yew::{html, Children, Component, ComponentLink, Html, Properties};

type ContextData = Option<Box<dyn Any>>;

thread_local! {
    // TODO consider using a stack approach instead of swapping the context data in and out of the providers all the time.
    static CURRENT_CONTEXT: RefCell<HashMap<TypeId, ContextData>> = RefCell::new(HashMap::new());
}

pub struct ContextProvider<T: Clone + 'static> {
    _never: std::marker::PhantomData<T>,
    context_data: RefCell<ContextData>,
    children: Children,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ContextProviderProps<T: Clone> {
    pub context: T,
    pub children: Children,
}

impl<T: Clone + 'static> Component for ContextProvider<T> {
    type Message = ();
    type Properties = ContextProviderProps<T>;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        ContextProvider {
            _never: std::marker::PhantomData::default(),
            context_data: RefCell::new(Some(Box::new(props.context))),
            children: props.children,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.context_data = RefCell::new(Some(Box::new(props.context)));
        self.children = props.children;
        true // TODO
    }

    fn view(&self) -> Html {
        // Place ourselves
        yew::services::ConsoleService::new().log("PLACING HOOK"); // TODO remove
        CURRENT_CONTEXT.with(|cell| {
            let mut current_contexts = cell.borrow_mut();
            let previous_ctx = current_contexts.entry(TypeId::of::<T>()).or_insert(None);
            std::mem::swap(self.context_data.borrow_mut().deref_mut(), previous_ctx);
        });

        return html! {
            <>
                { self.children.render() }
            </>
        };
    }

    fn rendered(&mut self, _first_render: bool) {
        // TODO remove log
        yew::services::ConsoleService::new().log("RECLAIMING HOOK");
        // Reclaim our context
        CURRENT_CONTEXT.with(|cell| {
            let mut current_contexts = cell.borrow_mut();
            let my_ctx = current_contexts.entry(TypeId::of::<T>()).or_default();
            std::mem::swap(self.context_data.borrow_mut().deref_mut(), my_ctx);
        });
    }
}

pub fn use_context<T: 'static + Clone>() -> Option<T> {
    CURRENT_CONTEXT.with(|cell| {
        let mut current_context = cell.borrow_mut();
        let context = current_context.entry(TypeId::of::<T>()).or_default();
        if let Some(context) = context {
            let context_ref = context.downcast_ref::<T>().expect("Data corruption");
            Some((*context_ref).clone())
        } else {
            None
        }
    })
}
