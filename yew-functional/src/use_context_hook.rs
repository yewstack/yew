// Naming this file use_context could be confusing. Not least to the IDE.
use std::any::Any;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::DerefMut;
use yew::html::Renderable;
use yew::{html, Children, Component, ComponentLink, Html, Properties};

thread_local! {
    // TODO using option means we do not have to modify the map much. Is this the right choice?
    static CURRENT_CONTEXT: RefCell<HashMap<TypeId, Option<Box<dyn Any>>>> = RefCell::new(HashMap::new());
}

pub struct ContextProvider<T: Clone + 'static> {
    _never: std::marker::PhantomData<T>,
    context_data: RefCell<Option<Box<dyn Any>>>,
    children: Children,
    link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ContextProviderProps<T: Clone> {
    pub context: T,
    pub children: Children,
}

impl<T: Clone + 'static> Component for ContextProvider<T> {
    type Message = ();
    type Properties = ContextProviderProps<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ContextProvider {
            _never: std::marker::PhantomData::default(),
            context_data: RefCell::new(Some(Box::new(props.context))),
            children: props.children,
            link,
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
        yew::services::ConsoleService::new().log("PLACING HOOK");
        CURRENT_CONTEXT.with(|cell| {
            let mut mutable_map = cell.borrow_mut();
            let previous_hook = mutable_map.entry(TypeId::of::<T>()).or_insert_with(|| None);
            std::mem::swap(self.context_data.borrow_mut().deref_mut(), previous_hook);
        });

        let rendered = self.children.render();

        // self.link.send_message(());

        return html! {
            <>
                {rendered}
            </>
        };
    }

    fn rendered(&mut self, _first_render: bool) {
        yew::services::ConsoleService::new().log("RECLAIMING HOOK");
        // Reclaim our context
        CURRENT_CONTEXT.with(|cell| {
            let mut mutable_map = cell.borrow_mut();
            let our_hook = mutable_map.entry(TypeId::of::<T>()).or_default();
            std::mem::swap(self.context_data.borrow_mut().deref_mut(), our_hook);
        });
    }
}

pub fn use_context<T: 'static + Clone>() -> Option<T> {
    CURRENT_CONTEXT.with(|cell| {
        let mut mutable_map = cell.borrow_mut();
        let context = mutable_map.entry(TypeId::of::<T>()).or_default();
        if let Some(context) = context {
            let context_ref = context.downcast_ref::<T>().expect("Data corruption");
            Some((*context_ref).clone())
        } else {
            None
        }
    })
}
