use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

use serde::de::DeserializeOwned;
use serde::Serialize;
use yew::prelude::*;
#[cfg(target_arch = "wasm32")]
use yew::suspense::Suspension;
use yew::suspense::SuspensionResult;
pub use yew_link_macro::linked_state;

/// A type that can be resolved on the server and transferred to the client.
///
/// Implementors declare an `Input` type and an `Error` type. The actual resolve
/// logic is provided separately via [`Resolver::register`] or the
/// [`#[linked_state]`](linked_state) macro.
pub trait LinkedState: Serialize + DeserializeOwned + Clone + 'static {
    /// The input/deps used to look up this state.
    type Input: Serialize + DeserializeOwned + PartialEq + Clone + fmt::Debug + 'static;

    /// Application-level error returned by a failed resolve.
    type Error: Serialize + DeserializeOwned + Clone + fmt::Debug + fmt::Display + 'static;
}

/// Server-side extension of [`LinkedState`] that provides a resolve function.
///
/// You normally don't implement this by hand — use the [`#[linked_state]`](linked_state)
/// attribute macro instead. The macro generates this impl (gated behind
/// `#[cfg(not(target_arch = "wasm32"))]`) and strips the server code from
/// WASM bundles automatically.
#[cfg(not(target_arch = "wasm32"))]
pub trait LinkedStateResolve: LinkedState {
    type Context: Send + Sync + 'static;

    fn resolve<'a>(
        ctx: &'a Self::Context,
        input: &'a Self::Input,
    ) -> impl Future<Output = Result<Self, Self::Error>> + Send + 'a;
}

/// An uninhabited error type that implements `Serialize`/`Deserialize`.
///
/// Used as the default `LinkedState::Error` when `type Error` is omitted from
/// the `#[linked_state]` macro. Unlike `std::convert::Infallible`, this type
/// satisfies the serde bounds required by the trait.
#[derive(Clone, Debug, Serialize, serde::Deserialize)]
pub enum Never {}

impl fmt::Display for Never {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {}
    }
}

/// Error returned by [`use_linked_state`].
///
/// Distinguishes application-level errors (from the resolve function) from
/// infrastructure failures (network, serialization, missing resolver).
#[derive(Serialize, serde::Deserialize, Clone, Debug)]
pub enum LinkError<E> {
    /// The resolve function returned an application-level error.
    Resolve(E),
    /// Infrastructure failure (network, serialization, missing resolver).
    Internal(String),
}

impl<E: fmt::Display> fmt::Display for LinkError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Resolve(e) => fmt::Display::fmt(e, f),
            Self::Internal(s) => f.write_str(s),
        }
    }
}

#[derive(Serialize, serde::Deserialize)]
pub struct LinkRequest {
    type_name: String,
    input: serde_json::Value,
}

#[derive(Serialize, serde::Deserialize)]
pub struct LinkResponse {
    #[serde(default)]
    ok: Option<serde_json::Value>,
    #[serde(default)]
    error: Option<serde_json::Value>,
}

type ResolveBoxFuture =
    Pin<Box<dyn Future<Output = Result<serde_json::Value, serde_json::Value>> + Send>>;
type ResolverFn = Box<dyn Fn(serde_json::Value) -> ResolveBoxFuture + Send + Sync>;

/// Registry of resolve functions, keyed by [`std::any::type_name`].
///
/// Constructed on the server and passed to [`LinkProvider`]. Also used by the
/// axum handler when the `axum` feature is enabled.
pub struct Resolver {
    handlers: HashMap<&'static str, ResolverFn>,
}

impl fmt::Debug for Resolver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Resolver")
            .field("types", &self.handlers.keys().collect::<Vec<_>>())
            .finish()
    }
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Register a resolver for `T`. The closure receives `T::Input` and returns
    /// a future that produces `Result<T, T::Error>`.
    pub fn register<T, F, Fut>(mut self, f: F) -> Self
    where
        T: LinkedState + Send,
        T::Error: Send,
        F: Fn(T::Input) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<T, T::Error>> + Send + 'static,
    {
        let type_name = std::any::type_name::<T>();
        self.handlers.insert(
            type_name,
            Box::new(move |input_json: serde_json::Value| {
                let input: T::Input = serde_json::from_value(input_json)
                    .expect("failed to deserialize linked state input");
                let fut = f(input);
                Box::pin(async move {
                    match fut.await {
                        Ok(val) => serde_json::to_value(&val)
                            .map_err(|e| serde_json::Value::String(e.to_string())),
                        Err(e) => Err(serde_json::to_value(&e).unwrap_or(serde_json::Value::Null)),
                    }
                })
            }),
        );
        self
    }

    /// Resolve a [`LinkRequest`].
    pub async fn resolve_request(
        &self,
        req: &LinkRequest,
    ) -> Result<serde_json::Value, serde_json::Value> {
        let handler = self.handlers.get(req.type_name.as_str()).ok_or_else(|| {
            serde_json::Value::String(format!("no resolver registered for {}", req.type_name))
        })?;
        handler(req.input.clone()).await
    }
}

impl Default for Resolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Resolver {
    /// Register a resolver for `T` using its [`LinkedStateResolve`] impl.
    ///
    /// The `ctx` is wrapped in an [`Arc`] internally so clones are cheap.
    pub fn register_linked<T>(self, ctx: T::Context) -> Self
    where
        T: LinkedStateResolve + Send + 'static,
        T::Input: Send,
        T::Error: Send,
    {
        let ctx = Arc::new(ctx);
        self.register::<T, _, _>(move |input| {
            let ctx = ctx.clone();
            async move { T::resolve(&*ctx, &input).await }
        })
    }
}

type Cache = Rc<RefCell<HashMap<String, serde_json::Value>>>;

#[cfg(target_arch = "wasm32")]
fn cache_key<T: LinkedState>(input: &T::Input) -> String {
    format!(
        "{}:{}",
        std::any::type_name::<T>(),
        serde_json::to_string(input).unwrap_or_default()
    )
}

#[derive(Clone)]
struct LinkContextInner {
    cache: Cache,
    endpoint: AttrValue,
    #[cfg(feature = "ssr")]
    resolver: Option<Arc<Resolver>>,
}

impl PartialEq for LinkContextInner {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.cache, &other.cache) && self.endpoint == other.endpoint
    }
}

impl LinkContextInner {
    #[cfg(target_arch = "wasm32")]
    async fn fetch_remote<T: LinkedState>(
        &self,
        input: &T::Input,
    ) -> Result<T, LinkError<T::Error>> {
        use gloo_net::http::Request;

        let req_body = LinkRequest {
            type_name: std::any::type_name::<T>().to_string(),
            input: serde_json::to_value(input).map_err(|e| LinkError::Internal(e.to_string()))?,
        };
        let resp = Request::post(self.endpoint.as_ref())
            .json(&req_body)
            .map_err(|e| LinkError::Internal(e.to_string()))?
            .send()
            .await
            .map_err(|e| LinkError::Internal(e.to_string()))?;

        let link_resp: LinkResponse = resp
            .json()
            .await
            .map_err(|e| LinkError::Internal(e.to_string()))?;

        match link_resp.ok {
            Some(val) => {
                serde_json::from_value(val).map_err(|e| LinkError::Internal(e.to_string()))
            }
            None => match link_resp.error {
                Some(err_val) => {
                    let e: T::Error = serde_json::from_value(err_val)
                        .map_err(|e| LinkError::Internal(e.to_string()))?;
                    Err(LinkError::Resolve(e))
                }
                None => Err(LinkError::Internal("unknown error".into())),
            },
        }
    }

    #[cfg(feature = "ssr")]
    async fn resolve_local<T: LinkedState>(
        &self,
        input: &T::Input,
    ) -> Result<T, LinkError<T::Error>> {
        let resolver = self
            .resolver
            .as_ref()
            .expect("resolver not set on server-side LinkProvider");
        let req = LinkRequest {
            type_name: std::any::type_name::<T>().to_string(),
            input: serde_json::to_value(input).map_err(|e| LinkError::Internal(e.to_string()))?,
        };
        match resolver.resolve_request(&req).await {
            Ok(val) => serde_json::from_value(val).map_err(|e| LinkError::Internal(e.to_string())),
            Err(err_val) => {
                let e: T::Error = serde_json::from_value(err_val)
                    .map_err(|e| LinkError::Internal(e.to_string()))?;
                Err(LinkError::Resolve(e))
            }
        }
    }
}

/// Wrapper so [`Resolver`] can be passed as a component prop.
///
/// Uses `Arc` internally so it is `Send` (required by `ServerRenderer::with_props`).
#[derive(Clone)]
pub struct ResolverProp(pub Arc<Resolver>);

impl fmt::Debug for ResolverProp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ResolverProp").field(&self.0).finish()
    }
}

impl PartialEq for ResolverProp {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for ResolverProp {}

impl From<Resolver> for ResolverProp {
    fn from(r: Resolver) -> Self {
        Self(Arc::new(r))
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct LinkProviderProps {
    pub children: Children,
    /// Remote endpoint URL used by the client to fetch linked states.
    #[prop_or_default]
    pub endpoint: AttrValue,
    /// Server-side resolver. Ignored on wasm32 targets.
    #[prop_or_default]
    pub resolver: Option<ResolverProp>,
}

/// Provides linked-state resolution context to descendant components.
///
/// On the server, pass a [`ResolverProp`] so that [`use_linked_state`] can
/// resolve states locally. On the client, pass an `endpoint` URL.
#[function_component]
pub fn LinkProvider(props: &LinkProviderProps) -> Html {
    let cache: Cache = (*use_ref(|| Rc::new(RefCell::new(HashMap::new())))).clone();

    let ctx = LinkContextInner {
        cache,
        endpoint: props.endpoint.clone(),
        #[cfg(feature = "ssr")]
        resolver: props.resolver.as_ref().map(|r| Arc::clone(&r.0)),
    };

    html! {
        <ContextProvider<LinkContextInner> context={ctx}>
            { for props.children.iter() }
        </ContextProvider<LinkContextInner>>
    }
}

/// Fetch a [`LinkedState`] value.
///
/// On the server the state is resolved locally via the [`Resolver`] registered
/// in the ancestor [`LinkProvider`], and the result is embedded in the SSR
/// HTML for zero-cost hydration on the client.
///
/// On the client during hydration, the SSR-embedded state is read directly
/// without any network request. On subsequent client-side navigations the
/// state is fetched from the provider's `endpoint` URL.
///
/// The hook suspends while the state is being resolved/fetched. Once resolved,
/// the inner `Result` carries either the value or a [`LinkError`].
///
/// # Panics
///
/// Panics if there is no ancestor [`LinkProvider`] in the component tree.
#[hook]
pub fn use_linked_state<T: LinkedState>(
    input: T::Input,
) -> SuspensionResult<Result<Rc<T>, LinkError<T::Error>>> {
    let _link_ctx =
        use_context::<LinkContextInner>().expect("use_linked_state requires a LinkProvider");

    type Prepared<T, E> = Result<T, LinkError<E>>;

    #[cfg(feature = "ssr")]
    {
        let prepared = {
            let link_ctx = _link_ctx.clone();
            yew::functional::use_prepared_state_with_suspension(
                input,
                move |input: Rc<T::Input>| {
                    let link_ctx = link_ctx.clone();
                    async move { link_ctx.resolve_local::<T>(&input).await }
                },
            )
        }?;

        let result: Rc<Prepared<T, T::Error>> =
            prepared.expect("prepared state should always be Some on SSR");
        Ok(match result.as_ref() {
            Ok(val) => Ok(Rc::new(val.clone())),
            Err(e) => Err(e.clone()),
        })
    }

    #[cfg(all(not(feature = "ssr"), not(target_arch = "wasm32")))]
    {
        let _prepared = yew::functional::use_prepared_state_with_suspension::<
            Prepared<T, T::Error>,
            T::Input,
        >(input)?;
        Ok(Err(LinkError::Internal(
            "yew-link requires the `ssr` feature (server) or a wasm32 target (client)".into(),
        )))
    }

    #[cfg(all(not(feature = "ssr"), target_arch = "wasm32"))]
    {
        let prepared =
            yew::functional::use_prepared_state::<Prepared<T, T::Error>, T::Input>(input.clone())?;

        if let Some(ref result) = prepared {
            let key = cache_key::<T>(&input);
            if let Ok(json_val) = serde_json::to_value(result.as_ref()) {
                _link_ctx.cache.borrow_mut().entry(key).or_insert(json_val);
            }
        }

        let render_epoch = use_state(|| 0u32);
        let suspension_ref = use_mut_ref(|| Option::<Suspension>::None);
        let fetch_id = use_mut_ref(|| 0u32);

        if let Some(result) = prepared {
            return Ok(match result.as_ref() {
                Ok(val) => Ok(Rc::new(val.clone())),
                Err(e) => Err(e.clone()),
            });
        }

        let key = cache_key::<T>(&input);
        if let Some(cached_val) = _link_ctx.cache.borrow().get(&key).cloned() {
            if let Ok(result) = serde_json::from_value::<Prepared<T, T::Error>>(cached_val) {
                return Ok(result.map(Rc::new));
            }
        }

        {
            let existing = suspension_ref.borrow();
            if let Some(sus) = &*existing {
                if !sus.resumed() {
                    return Err(sus.clone());
                }
            }
        }

        let id = {
            let mut fid = fetch_id.borrow_mut();
            *fid = fid.wrapping_add(1);
            *fid
        };

        let sus = Suspension::from_future({
            let link_ctx = _link_ctx.clone();
            let fetch_id = fetch_id.clone();
            let render_epoch = render_epoch.clone();
            async move {
                let result: Result<T, LinkError<T::Error>> =
                    link_ctx.fetch_remote::<T>(&input).await;

                if *fetch_id.borrow() != id {
                    return;
                }

                let should_cache = match &result {
                    Ok(_) => true,
                    Err(LinkError::Resolve(_)) => true,
                    Err(LinkError::Internal(_)) => false,
                };
                if should_cache {
                    if let Ok(json_val) = serde_json::to_value(&result) {
                        link_ctx.cache.borrow_mut().insert(key, json_val);
                    }
                }
                render_epoch.set(id);
            }
        });

        *suspension_ref.borrow_mut() = Some(sus.clone());
        Err(sus)
    }
}

#[cfg(all(feature = "axum", not(target_arch = "wasm32")))]
pub mod service {
    use std::sync::Arc;

    use axum::extract::State;
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    use axum::Json;

    use super::{LinkRequest, LinkResponse, Resolver};

    /// Axum handler that resolves [`LinkRequest`]s.
    ///
    /// ```ignore
    /// let resolver = Arc::new(
    ///     Resolver::new()
    ///         .register::<Post>(|id| async move { db::get_post(id).await })
    /// );
    ///
    /// let app = axum::Router::new()
    ///     .route("/api/link", axum::routing::post(linked_state_handler))
    ///     .with_state(resolver);
    /// ```
    pub async fn linked_state_handler(
        State(resolver): State<Arc<Resolver>>,
        Json(req): Json<LinkRequest>,
    ) -> impl IntoResponse {
        match resolver.resolve_request(&req).await {
            Ok(val) => (
                StatusCode::OK,
                Json(LinkResponse {
                    ok: Some(val),
                    error: None,
                }),
            ),
            Err(err_val) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(LinkResponse {
                    ok: None,
                    error: Some(err_val),
                }),
            ),
        }
    }
}

#[cfg(all(feature = "axum", not(target_arch = "wasm32")))]
pub use service::linked_state_handler;
