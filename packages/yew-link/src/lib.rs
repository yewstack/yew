use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
#[cfg(target_arch = "wasm32")]
use std::collections::HashSet;
use std::fmt;
use std::hash::Hash;
#[cfg(target_arch = "wasm32")]
use std::num::NonZeroUsize;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
use lru::LruCache;
use serde::Serialize;
use serde::de::DeserializeOwned;
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
    type Input: Serialize + DeserializeOwned + PartialEq + Eq + Hash + Clone + fmt::Debug + 'static;

    /// Application-level error returned by a failed resolve.
    type Error: Serialize + DeserializeOwned + Clone + fmt::Debug + fmt::Display + 'static;

    /// Stable wire-format key used to route requests between client and server.
    ///
    /// Generated automatically by [`#[linked_state]`](linked_state) as
    /// `concat!(module_path!(), "::", stringify!(Type))`. If you implement
    /// `LinkedState` manually (e.g. for generic types), set this to a
    /// string that is identical across server and client builds.
    const TYPE_KEY: &'static str;
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

/// Handle returned by [`use_linked_state`].
///
/// Provides access to the resolved data, a [`refresh`](Self::refresh)
/// method that triggers a background re-fetch, and an
/// [`is_refreshing`](Self::is_refreshing) method to check whether a refresh
/// is in progress (stale-while-revalidate).
#[derive(Clone)]
pub struct LinkedStateHandle<T: LinkedState> {
    result: Result<Rc<T>, LinkError<T::Error>>,
    refresh: Callback<()>,
    refreshing: bool,
}

impl<T: LinkedState> LinkedStateHandle<T> {
    /// Returns the resolved value, panicking if the resolver returned an error.
    ///
    /// This clones the inner [`Rc`], which is cheap.
    ///
    /// # Panics
    ///
    /// Panics if the resolver returned a [`LinkError`]. Use
    /// [`as_result`](Self::as_result) for non-panicking access.
    pub fn data(&self) -> Rc<T> {
        self.result.as_ref().unwrap().clone()
    }

    /// Returns a reference to the underlying result.
    pub fn as_result(&self) -> &Result<Rc<T>, LinkError<T::Error>> {
        &self.result
    }

    /// Triggers a background re-fetch of this linked state.
    ///
    /// Unlike a full suspend, the component keeps displaying the previous
    /// (stale) value while the fresh data is being fetched. Use
    /// [`is_refreshing`](Self::is_refreshing) to show a loading indicator
    /// alongside the stale data.
    ///
    /// On the server this is a no-op.
    pub fn refresh(&self) {
        self.refresh.emit(());
    }

    /// Returns `true` while a background refresh is in progress and this
    /// handle still holds the previous (stale) value.
    pub fn is_refreshing(&self) -> bool {
        self.refreshing
    }
}

#[doc(hidden)]
#[derive(Serialize, serde::Deserialize)]
pub struct LinkRequest {
    type_key: String,
    input: serde_json::Value,
}

#[doc(hidden)]
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
        self.handlers.insert(
            T::TYPE_KEY,
            Box::new(move |input_json: serde_json::Value| {
                let input: T::Input = match serde_json::from_value(input_json) {
                    Ok(v) => v,
                    Err(e) => {
                        return Box::pin(async move {
                            Err(serde_json::Value::String(format!(
                                "failed to deserialize input: {e}"
                            )))
                        });
                    }
                };
                let fut = f(input);
                Box::pin(async move {
                    match fut.await {
                        Ok(val) => serde_json::to_value(&val)
                            .map_err(|e| serde_json::Value::String(e.to_string())),
                        Err(e) => Err(serde_json::to_value(&e).unwrap_or_else(|ser_err| {
                            serde_json::Value::String(format!(
                                "{e}: (serialization failed: {ser_err})"
                            ))
                        })),
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
        let handler = self.handlers.get(req.type_key.as_str()).ok_or_else(|| {
            serde_json::Value::String(format!("no resolver registered for {}", req.type_key))
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

#[derive(Clone)]
struct CacheKey {
    type_id: TypeId,
    input_hash: u64,
    input: Rc<dyn Any>,
    eq_fn: fn(&dyn Any, &dyn Any) -> bool,
}

impl Hash for CacheKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.type_id.hash(state);
        self.input_hash.hash(state);
    }
}

impl PartialEq for CacheKey {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id && (self.eq_fn)(&*self.input, &*other.input)
    }
}

impl Eq for CacheKey {}

#[cfg(all(not(feature = "ssr"), target_arch = "wasm32"))]
fn eq_inputs<I: PartialEq + 'static>(a: &dyn Any, b: &dyn Any) -> bool {
    a.downcast_ref::<I>()
        .zip(b.downcast_ref::<I>())
        .is_some_and(|(a, b)| a == b)
}

#[cfg(all(not(feature = "ssr"), target_arch = "wasm32"))]
fn cache_key<T: LinkedState>(input: &T::Input) -> CacheKey {
    use std::hash::Hasher;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    input.hash(&mut hasher);
    CacheKey {
        type_id: TypeId::of::<T>(),
        input_hash: hasher.finish(),
        input: Rc::new(input.clone()),
        eq_fn: eq_inputs::<T::Input>,
    }
}

#[cfg(target_arch = "wasm32")]
type Cache = Rc<RefCell<LruCache<CacheKey, serde_json::Value>>>;
#[cfg(not(target_arch = "wasm32"))]
type Cache = Rc<RefCell<HashMap<CacheKey, serde_json::Value>>>;

#[cfg(target_arch = "wasm32")]
type InFlight = Rc<RefCell<HashMap<CacheKey, Suspension>>>;

#[cfg(target_arch = "wasm32")]
type Refreshing = Rc<RefCell<HashSet<CacheKey>>>;

#[derive(Clone)]
struct LinkContextInner {
    cache: Cache,
    #[cfg(target_arch = "wasm32")]
    in_flight: InFlight,
    #[cfg(target_arch = "wasm32")]
    refreshing: Refreshing,
    endpoint: AttrValue,
    #[cfg(feature = "ssr")]
    resolver: Option<Arc<Resolver>>,
}

impl PartialEq for LinkContextInner {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.cache, &other.cache) && self.endpoint == other.endpoint && {
            #[cfg(target_arch = "wasm32")]
            {
                Rc::ptr_eq(&self.in_flight, &other.in_flight)
                    && Rc::ptr_eq(&self.refreshing, &other.refreshing)
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                true
            }
        }
    }
}

impl LinkContextInner {
    #[cfg(all(not(feature = "ssr"), target_arch = "wasm32"))]
    async fn fetch_remote<T: LinkedState>(
        &self,
        input: &T::Input,
    ) -> Result<T, LinkError<T::Error>> {
        use gloo_net::http::Request;

        let req_body = LinkRequest {
            type_key: T::TYPE_KEY.to_string(),
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
            type_key: T::TYPE_KEY.to_string(),
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
    /// Maximum number of entries in the linked-state cache. Defaults to 64.
    #[prop_or(64)]
    pub cache_capacity: usize,
}

/// Provides linked-state resolution context to descendant components.
///
/// On the server, pass a [`ResolverProp`] so that [`use_linked_state`] can
/// resolve states locally. On the client, pass an `endpoint` URL.
#[component]
pub fn LinkProvider(props: &LinkProviderProps) -> Html {
    #[cfg(target_arch = "wasm32")]
    let cache: Cache = {
        let cap = NonZeroUsize::new(props.cache_capacity).unwrap_or(NonZeroUsize::MIN);
        (*use_ref(|| Rc::new(RefCell::new(LruCache::new(cap))))).clone()
    };
    #[cfg(not(target_arch = "wasm32"))]
    let cache: Cache = (*use_ref(|| Rc::new(RefCell::new(HashMap::new())))).clone();
    #[cfg(target_arch = "wasm32")]
    let in_flight: InFlight = (*use_ref(|| Rc::new(RefCell::new(HashMap::new())))).clone();
    #[cfg(target_arch = "wasm32")]
    let refreshing: Refreshing = (*use_ref(|| Rc::new(RefCell::new(HashSet::new())))).clone();

    let ctx = LinkContextInner {
        cache,
        #[cfg(target_arch = "wasm32")]
        in_flight,
        #[cfg(target_arch = "wasm32")]
        refreshing,
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
/// The hook suspends while the state is being resolved/fetched for the first
/// time. On [`refresh`](LinkedStateHandle::refresh), the previous value is
/// kept visible (stale-while-revalidate) and
/// [`is_refreshing`](LinkedStateHandle::is_refreshing) returns `true` until
/// the fresh data arrives.
///
/// Multiple components requesting the same `(T, Input)` concurrently share a
/// single in-flight request.
///
/// # Panics
///
/// Panics if there is no ancestor [`LinkProvider`] in the component tree.
#[hook]
pub fn use_linked_state<T: LinkedState>(input: T::Input) -> SuspensionResult<LinkedStateHandle<T>> {
    #[cfg(any(feature = "ssr", target_arch = "wasm32"))]
    let link_ctx =
        use_context::<LinkContextInner>().expect("use_linked_state requires a LinkProvider");

    #[cfg(any(feature = "ssr", target_arch = "wasm32"))]
    type Prepared<T, E> = Result<T, LinkError<E>>;

    #[cfg(feature = "ssr")]
    {
        let prepared = {
            let link_ctx = link_ctx.clone();
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
        Ok(LinkedStateHandle {
            result: match result.as_ref() {
                Ok(val) => Ok(Rc::new(val.clone())),
                Err(e) => Err(e.clone()),
            },
            refresh: Callback::from(|_: ()| {}),
            refreshing: false,
        })
    }

    #[cfg(all(not(feature = "ssr"), not(target_arch = "wasm32")))]
    {
        let _ = input;
        Ok(LinkedStateHandle {
            result: Err(LinkError::Internal(
                "yew-link requires the `ssr` feature (server) or a wasm32 target (client)".into(),
            )),
            refresh: Callback::from(|_: ()| {}),
            refreshing: false,
        })
    }

    #[cfg(all(not(feature = "ssr"), target_arch = "wasm32"))]
    {
        let prepared =
            yew::functional::use_prepared_state::<Prepared<T, T::Error>, T::Input>(input.clone())?;

        let key = cache_key::<T>(&input);

        if let Some(ref result) = prepared {
            if let Ok(json_val) = serde_json::to_value(result.as_ref()) {
                let mut cache = link_ctx.cache.borrow_mut();
                if cache.peek(&key).is_none() {
                    cache.put(key.clone(), json_val);
                }
            }
        }

        let force_update = use_force_update();
        let has_refreshed = use_ref(|| std::cell::Cell::new(false));

        let refresh = {
            let cache = link_ctx.cache.clone();
            let refreshing = link_ctx.refreshing.clone();
            let key = key.clone();
            let link_ctx = link_ctx.clone();
            let input = input.clone();
            let force_update = force_update.clone();
            let has_refreshed = has_refreshed.clone();
            Callback::from(move |()| {
                refreshing.borrow_mut().insert(key.clone());
                has_refreshed.set(true);

                let link_ctx = link_ctx.clone();
                let key = key.clone();
                let cache = cache.clone();
                let refreshing = refreshing.clone();
                let inner_force_update = force_update.clone();
                let input = input.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let result: Result<T, LinkError<T::Error>> =
                        link_ctx.fetch_remote::<T>(&input).await;

                    refreshing.borrow_mut().remove(&key);

                    let should_cache = match &result {
                        Ok(_) | Err(LinkError::Resolve(_)) => true,
                        Err(LinkError::Internal(_)) => false,
                    };
                    if should_cache {
                        if let Ok(json_val) = serde_json::to_value(&result) {
                            cache.borrow_mut().put(key, json_val);
                        }
                    }

                    inner_force_update.force_update();
                });

                force_update.force_update();
            })
        };

        if !has_refreshed.get() {
            if let Some(result) = prepared {
                return Ok(LinkedStateHandle {
                    result: match result.as_ref() {
                        Ok(val) => Ok(Rc::new(val.clone())),
                        Err(e) => Err(e.clone()),
                    },
                    refresh,
                    refreshing: false,
                });
            }
        }

        let is_refreshing = link_ctx.refreshing.borrow().contains(&key);

        if let Some(cached_val) = link_ctx.cache.borrow_mut().get(&key).cloned() {
            if let Ok(result) = serde_json::from_value::<Prepared<T, T::Error>>(cached_val) {
                return Ok(LinkedStateHandle {
                    result: result.map(Rc::new),
                    refresh,
                    refreshing: is_refreshing,
                });
            }
        }

        if let Some(sus) = link_ctx.in_flight.borrow().get(&key).cloned() {
            if !sus.resumed() {
                return Err(sus);
            }
            if let Some(cached_val) = link_ctx.cache.borrow_mut().get(&key).cloned() {
                if let Ok(result) = serde_json::from_value::<Prepared<T, T::Error>>(cached_val) {
                    return Ok(LinkedStateHandle {
                        result: result.map(Rc::new),
                        refresh,
                        refreshing: false,
                    });
                }
            }
        }

        let sus = Suspension::from_future({
            let link_ctx = link_ctx.clone();
            let key = key.clone();
            async move {
                let result: Result<T, LinkError<T::Error>> =
                    link_ctx.fetch_remote::<T>(&input).await;

                link_ctx.in_flight.borrow_mut().remove(&key);

                let should_cache = match &result {
                    Ok(_) | Err(LinkError::Resolve(_)) => true,
                    Err(LinkError::Internal(_)) => false,
                };
                if should_cache {
                    if let Ok(json_val) = serde_json::to_value(&result) {
                        link_ctx.cache.borrow_mut().put(key, json_val);
                    }
                }
            }
        });

        link_ctx.in_flight.borrow_mut().insert(key, sus.clone());
        Err(sus)
    }
}

#[cfg(all(not(target_arch = "wasm32"), any(feature = "axum", feature = "actix")))]
mod services;

#[cfg(all(not(target_arch = "wasm32"), any(feature = "axum", feature = "actix")))]
pub use services::*;
