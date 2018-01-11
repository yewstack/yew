use std::rc::Rc;
use std::cell::RefCell;
use stdweb::web::document;
use html::{Component, ComponentUpdate, Html, Callback, ScopeBuilder, ScopeSender, ScopeRef, ShouldUpdate};
use std::ops::{Deref, DerefMut};

pub struct App<CTX, MOD, MSG>
where
    CTX: 'static,
    MOD: 'static,
    MSG: 'static,
{
    builder: ScopeBuilder<AppContext<CTX, MOD, MSG>, AppImpl<CTX, MOD, MSG>>,
}

pub struct AppSender<CTX, MOD, MSG>
where
    CTX: 'static,
    MOD: 'static,
    MSG: 'static,
{
    sender: ScopeSender<AppContext<CTX, MOD, MSG>, AppImpl<CTX, MOD, MSG>>,
}

impl<CTX, MOD, MSG> AppSender<CTX, MOD, MSG> {
    pub fn send(&mut self, msg: MSG) {
        self.sender.send(ComponentUpdate::Message(msg))
    }
}

impl<CTX, MOD, MSG> App<CTX, MOD, MSG> {

    pub fn new() -> Self {
        App {
            builder: ScopeBuilder::new(),
        }
    }

    pub fn sender(&mut self) -> AppSender<CTX, MOD, MSG> {
        let sender = self.builder.sender();
        AppSender {
            sender,
        }
    }

    pub fn mount<U, V>(self, context: CTX, model: MOD, update: U, view: V)
    where
        U: Fn(&mut AppContext<CTX, MOD, MSG>, &mut MOD, MSG) -> ShouldUpdate + 'static,
        V: Fn(&MOD) -> Html<AppContext<CTX, MOD, MSG>, AppImpl<CTX, MOD, MSG>> + 'static,
    {
        self.mount_to("body", context, model, update, view)
    }

    pub fn mount_to<U, V>(self, selector: &str, context: CTX, model: MOD, update: U, view: V)
    where
        U: Fn(&mut AppContext<CTX, MOD, MSG>, &mut MOD, MSG) -> ShouldUpdate + 'static,
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


pub struct AppImpl<CTX, MOD, MSG>
where
    CTX: 'static,
    MOD: 'static,
    MSG: 'static,
{
    model: MOD,
    update: Box<Fn(&mut AppContext<CTX, MOD, MSG>, &mut MOD, MSG) -> ShouldUpdate>,
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

    fn update(&mut self, msg: Self::Msg, context: &mut ScopeRef<AppContext<CTX, MOD, MSG>, Self>) -> ShouldUpdate {
        (self.update)(&mut *context, &mut self.model, msg)
    }

    fn view(&self) -> Html<AppContext<CTX, MOD, MSG>, Self> {
        (self.view)(&self.model)
    }
}

pub type AppHtml<CTX, MOD, MSG> = Html<AppContext<CTX, MOD, MSG>, AppImpl<CTX, MOD, MSG>>;
