//! Starting module to start building a yew application.

use std::rc::Rc;
use std::cell::RefCell;
use stdweb::web::document;
use html::{Component, ComponentUpdate, Html, Callback, ScopeBuilder, ScopeSender, ScopeRef, ShouldRender};
use std::ops::{Deref, DerefMut};

/// App is a wrapper to hold any types as: context, model and message.
/// It joins instances of these types to a component.
pub struct App<CTX, MOD, MSG>
where
    CTX: 'static,
    MOD: 'static,
    MSG: 'static,
{
    builder: ScopeBuilder<AppContext<CTX, MOD, MSG>, AppImpl<CTX, MOD, MSG>>,
}

/// This class keeps a sender to a component to send a messages to a loop
/// and to schedule the next update call.
pub struct AppSender<CTX, MOD, MSG>
where
    CTX: 'static,
    MOD: 'static,
    MSG: 'static,
{
    sender: ScopeSender<AppContext<CTX, MOD, MSG>, AppImpl<CTX, MOD, MSG>>,
}

impl<CTX, MOD, MSG> AppSender<CTX, MOD, MSG> {
    /// Send the message and schedule an update.
    pub fn send(&mut self, msg: MSG) {
        self.sender.send(ComponentUpdate::Message(msg))
    }
}

impl<CTX, MOD, MSG> App<CTX, MOD, MSG> {

    /// Creates a new app with an attached scope builder.
    pub fn new() -> Self {
        App {
            builder: ScopeBuilder::new(),
        }
    }

    /// Returns a cloned sender.
    pub fn sender(&mut self) -> AppSender<CTX, MOD, MSG> {
        let sender = self.builder.sender();
        AppSender {
            sender,
        }
    }

    /// Alias to `mount_to("body", ...)`.
    pub fn mount<U, V>(self, context: CTX, model: MOD, update: U, view: V)
    where
        U: Fn(&mut AppContext<CTX, MOD, MSG>, &mut MOD, MSG) -> ShouldRender + 'static,
        V: Fn(&MOD) -> Html<AppContext<CTX, MOD, MSG>, AppImpl<CTX, MOD, MSG>> + 'static,
    {
        self.mount_to("body", context, model, update, view)
    }

    /// The main entrypoint of an isolated yew program. It works similar as `program`
    /// function in Elm. You should provide an initial model, `update` function
    /// which will update the state of the model and a `view` function which
    /// will render the model to a virtual DOM tree.
    pub fn mount_to<U, V>(self, selector: &str, context: CTX, model: MOD, update: U, view: V)
    where
        U: Fn(&mut AppContext<CTX, MOD, MSG>, &mut MOD, MSG) -> ShouldRender + 'static,
        V: Fn(&MOD) -> Html<AppContext<CTX, MOD, MSG>, AppImpl<CTX, MOD, MSG>> + 'static,
    {
        let element = document().query_selector(selector)
            .expect(format!("can't get node with selector `{}` for rendering", selector).as_str());
        let app_impl = AppImpl {
            model: model,
            update: Box::new(update),
            view: Box::new(view),
        };
        let sender = self.builder.sender();
        let context_impl = AppContext {
            app: Some(app_impl),
            sender,
            context,
        };
        let context = Rc::new(RefCell::new(context_impl));
        let scope = self.builder.build(context);
        scope.mount(element);
    }
}

/// Context holder which keeps references to a context instance and
/// a sender to a component bootstrapped from any model.
pub struct AppContext<CTX, MOD, MSG>
where
    CTX: 'static,
    MOD: 'static,
    MSG: 'static,
{
    app: Option<AppImpl<CTX, MOD, MSG>>,
    sender: ScopeSender<AppContext<CTX, MOD, MSG>, AppImpl<CTX, MOD, MSG>>,
    context: CTX,
}

impl<CTX, MOD, MSG> AsRef<CTX> for AppContext<CTX, MOD, MSG> {
    fn as_ref(&self) -> &CTX {
        &self.context
    }
}

impl<CTX, MOD, MSG> AsMut<CTX> for AppContext<CTX, MOD, MSG> {
    fn as_mut(&mut self) -> &mut CTX {
        &mut self.context
    }
}

impl<CTX, MOD, MSG> Deref for AppContext<CTX, MOD, MSG> {
    type Target = CTX;
    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl<CTX, MOD, MSG> DerefMut for AppContext<CTX, MOD, MSG> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.context
    }
}

impl<CTX, MOD, MSG> AppContext<CTX, MOD, MSG> {
    /// Schedules seding of a message back to `update` function.
    /// Returns callback which should be called by call `.emit()`
    /// with input parameter.
    pub fn send_back<F, IN>(&mut self, func: F) -> Callback<IN>
    where
        F: Fn(IN) -> MSG + 'static,
    {
        let sender = self.sender.clone();
        let callback = move |arg| {
            let msg = func(arg);
            sender.clone().send(ComponentUpdate::Message(msg));
        };
        callback.into()
    }
}

/// Generic component which can use different static types as: context, model, message.
pub struct AppImpl<CTX, MOD, MSG>
where
    CTX: 'static,
    MOD: 'static,
    MSG: 'static,
{
    model: MOD,
    update: Box<Fn(&mut AppContext<CTX, MOD, MSG>, &mut MOD, MSG) -> ShouldRender>,
    view: Box<Fn(&MOD) -> Html<AppContext<CTX, MOD, MSG>, AppImpl<CTX, MOD, MSG>>>,
}

impl<CTX, MOD, MSG> Component<AppContext<CTX, MOD, MSG>> for AppImpl<CTX, MOD, MSG>
where
    CTX: 'static,
    MOD: 'static,
    MSG: 'static,
{
    type Msg = MSG;
    type Properties = ();

    fn create(context: &mut ScopeRef<AppContext<CTX, MOD, MSG>, Self>) -> Self {
        context.app.take().expect("tried to unpack app impl twice")
    }

    fn update(&mut self, msg: Self::Msg, context: &mut ScopeRef<AppContext<CTX, MOD, MSG>, Self>) -> ShouldRender {
        (self.update)(&mut *context, &mut self.model, msg)
    }

    fn view(&self) -> Html<AppContext<CTX, MOD, MSG>, Self> {
        (self.view)(&self.model)
    }
}

/// A type which expected as a result of `view` function implementation.
pub type AppHtml<CTX, MOD, MSG> = Html<AppContext<CTX, MOD, MSG>, AppImpl<CTX, MOD, MSG>>;
