//! Router Component.
use std::borrow::Cow;
use std::rc::Rc;

use gloo::history::query::Raw;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

use crate::history::{AnyHistory, BrowserHistory, HashHistory, History, Location};
use crate::navigator::Navigator;
use crate::utils::{base_url, strip_slash_suffix};

/// Props for [`Router`].
#[derive(Properties, PartialEq, Clone)]
pub struct RouterProps {
    #[prop_or_default]
    pub children: Html,
    pub history: AnyHistory,
    #[prop_or_default]
    pub basename: Option<AttrValue>,
}

#[derive(Clone)]
pub(crate) struct LocationContext {
    location: Location,
    // Counter to force update.
    ctr: u32,
}

impl LocationContext {
    pub fn location(&self) -> Location {
        self.location.clone()
    }
}

impl PartialEq for LocationContext {
    fn eq(&self, rhs: &Self) -> bool {
        self.ctr == rhs.ctr
    }
}

impl Reducible for LocationContext {
    type Action = Location;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Self {
            location: action,
            ctr: self.ctr + 1,
        }
        .into()
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct NavigatorContext {
    navigator: Navigator,
}

impl NavigatorContext {
    pub fn navigator(&self) -> Navigator {
        self.navigator.clone()
    }
}

/// The base router.
///
/// The implementation is separated to make sure <Router /> has the same virtual dom layout as
/// the <BrowserRouter /> and <HashRouter />.
#[function_component(BaseRouter)]
fn base_router(props: &RouterProps) -> Html {
    let RouterProps {
        history,
        children,
        basename,
    } = props.clone();

    let basename = basename.map(|m| strip_slash_suffix(&m).to_owned());
    let navigator = Navigator::new(history.clone(), basename.clone());

    let old_basename = use_mut_ref(|| Option::<String>::None);
    let mut old_basename = old_basename.borrow_mut();
    if basename != *old_basename {
        // If `old_basename` is `Some`, path is probably prefixed with `old_basename`.
        // If `old_basename` is `None`, path may or may not be prefixed with the new `basename`,
        // depending on whether this is the first render.
        let old_navigator = Navigator::new(
            history.clone(),
            old_basename.as_ref().or(basename.as_ref()).cloned(),
        );
        *old_basename = basename.clone();
        let location = history.location();
        let stripped = old_navigator.strip_basename(Cow::from(location.path()));
        let prefixed = navigator.prefix_basename(&stripped);

        if prefixed != location.path() {
            history
                .replace_with_query(prefixed, Raw(location.query_str()))
                .unwrap_or_else(|never| match never {});
        } else {
            // Reaching here is possible if the page loads with the correct path, including the
            // initial basename. In that case, the new basename would be stripped and then
            // prefixed right back. While replacing the history would probably be harmless,
            // we might as well avoid doing it.
        }
    }

    let navi_ctx = NavigatorContext { navigator };

    let loc_ctx = use_reducer(|| LocationContext {
        location: history.location(),
        ctr: 0,
    });

    {
        let loc_ctx_dispatcher = loc_ctx.dispatcher();

        use_effect_with(history, move |history| {
            let history = history.clone();
            // Force location update when history changes.
            loc_ctx_dispatcher.dispatch(history.location());

            let history_cb = {
                let history = history.clone();
                move || loc_ctx_dispatcher.dispatch(history.location())
            };

            let listener = history.listen(history_cb);

            // We hold the listener in the destructor.
            move || {
                std::mem::drop(listener);
            }
        });
    }

    html! {
        <ContextProvider<NavigatorContext> context={navi_ctx}>
            <ContextProvider<LocationContext> context={(*loc_ctx).clone()}>
                {children}
            </ContextProvider<LocationContext>>
        </ContextProvider<NavigatorContext>>
    }
}

/// The Router component.
///
/// This provides location and navigator context to its children and switches.
///
/// If you are building a web application, you may want to consider using [`BrowserRouter`] instead.
///
/// You only need one `<Router />` for each application.
#[function_component(Router)]
pub fn router(props: &RouterProps) -> Html {
    html! {
        <BaseRouter ..{props.clone()} />
    }
}

/// Props for [`BrowserRouter`] and [`HashRouter`].
#[derive(Properties, PartialEq, Clone)]
pub struct ConcreteRouterProps {
    pub children: Html,
    #[prop_or_default]
    pub basename: Option<AttrValue>,
}

/// A [`Router`] that provides location information and navigator via [`BrowserHistory`].
///
/// This Router uses browser's native history to manipulate session history
/// and uses regular URL as route.
///
/// # Note
///
/// The router will by default use the value declared in `<base href="..." />` as its basename.
/// You may also specify a different basename with props.
#[function_component(BrowserRouter)]
pub fn browser_router(props: &ConcreteRouterProps) -> Html {
    let ConcreteRouterProps { children, basename } = props.clone();
    let history = use_state(|| AnyHistory::from(BrowserHistory::new()));

    // We acknowledge based in `<base href="..." />`
    let basename = basename.map(|m| m.to_string()).or_else(base_url);

    html! {
        <BaseRouter history={(*history).clone()} {basename}>
            {children}
        </BaseRouter>
    }
}

/// A [`Router`] that provides location information and navigator via [`HashHistory`].
///
/// This Router uses browser's native history to manipulate session history
/// and stores route in hash fragment.
///
/// # Warning
///
/// Prefer [`BrowserRouter`] whenever possible and use this as a last resort.
#[function_component(HashRouter)]
pub fn hash_router(props: &ConcreteRouterProps) -> Html {
    let ConcreteRouterProps { children, basename } = props.clone();
    let history = use_state(|| AnyHistory::from(HashHistory::new()));

    html! {
        <BaseRouter history={(*history).clone()} {basename}>
            {children}
        </BaseRouter>
    }
}
