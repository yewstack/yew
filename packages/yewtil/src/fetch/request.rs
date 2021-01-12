use crate::fetch::FetchError;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::marker::PhantomData;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, Window};

/// An enum representing what method to use for the request,
/// as well as a body if the method is able to have a body.
///
/// Connect, Options, Trace are omitted because they are unlikely to be used in this scenario.
/// Please open an issue if their absence is a problem for you.
pub enum MethodBody<'a, T> {
    Head,
    Get,
    Delete,
    Post(&'a T),
    Put(&'a T),
    Patch(&'a T),
}

impl<'a, T> MethodBody<'a, T> {
    pub fn as_method(&self) -> &'static str {
        match self {
            MethodBody::Get => "GET",
            MethodBody::Delete => "DELETE",
            MethodBody::Post(_) => "POST",
            MethodBody::Put(_) => "PUT",
            MethodBody::Patch(_) => "PATCH",
            MethodBody::Head => "HEAD",
        }
    }
}

impl<'a, T: Serialize> MethodBody<'a, T> {
    pub fn as_body<FORMAT: Format>(&self) -> Result<Option<JsValue>, FetchError> {
        let body: Option<String> = match self {
            MethodBody::Get | MethodBody::Delete | MethodBody::Head => None,
            MethodBody::Put(data) | MethodBody::Post(data) | MethodBody::Patch(data) => {
                let body =
                    FORMAT::serialize(data).ok_or(FetchError::CouldNotSerializeRequestBody)?;
                Some(body)
            }
        };

        let body = body.map(|data| JsValue::from_str(data.as_str()));
        Ok(body)
    }
}

// TODO, this is only works with String/&str. It would be a good addition if  Vec<u8>/&[u8] were supported.
/// Determines what format the data will be transmitted in.
pub trait Format {
    fn serialize<T: Serialize>(t: &T) -> Option<String>;
    fn deserialize<T: DeserializeOwned>(s: &str) -> Option<T>;
}

/// Transport data using the JSON format
pub struct Json;
impl Format for Json {
    //    type Transport = Text;

    fn serialize<T: Serialize>(t: &T) -> Option<String> {
        serde_json::to_string(t).ok()
    }

    fn deserialize<T: DeserializeOwned>(s: &str) -> Option<T> {
        serde_json::from_str(s).ok()
    }
}

/// Trait used to declare how a fetch request shall be made using a type.
///
///
/// Because it would be of little benefit to have to implement all details of this trait for every
/// request you make, the following should provide a template for reducing the amount of boilerplate
/// per request.
///
/// # Simplifying Example
/// ```no_run
/// use yewtil::fetch::{FetchRequest, MethodBody, Json, Fetch};
/// use serde::Serialize;
/// use serde::de::DeserializeOwned;
///
/// pub trait MyFetchRequest {
///     type RequestBody: Serialize;
///     type ResponseBody: DeserializeOwned;
///     fn path(&self) -> String;
///     fn method(&self) -> MethodBody<Self::RequestBody>;
/// }
/// /// A wrapper to allow implementing a foreign trait generically for anything wrapped by it that meets
/// /// the specified type bounds.
/// /// This isn't ideal, and will not be required in the future after coherence rules are changed.
/// /// https://github.com/rust-lang/rfcs/blob/master/text/2451-re-rebalancing-coherence.md
/// pub struct LocalWrapper<T>(T);
///
/// impl <T: MyFetchRequest> FetchRequest for LocalWrapper<T> {
///     type RequestBody = T::RequestBody;
///     type ResponseBody = T::ResponseBody;
///     type Format = Json; // Always serialize using json
///
///     fn url(&self) -> String {
///         // The origin will always be the same
///         format!("http://some_host_website.com/{}", self.0.path())
///     }
///
///     fn method(&self) -> MethodBody<Self::RequestBody> {
///         self.0.method()
///     }
///
///     fn headers(&self) -> Vec<(String, String)> {
///         // Always attach the same headers.
///         vec![
///             ("Content-Type".to_string(), "application/json".to_string())
///         ]
///     }
/// }
///
/// pub struct ApplesRequest;
/// impl MyFetchRequest for ApplesRequest {
///     type RequestBody = (); type ResponseBody = ();
///     fn path(&self) -> String { "apples".to_string()}
///     fn method(&self) -> MethodBody<Self::RequestBody> {MethodBody::Get}
/// }
///
/// pub enum Msg {
///     Variant
/// }
///
/// let fetch_wrapper = Fetch::new(LocalWrapper(ApplesRequest));
/// fetch_wrapper.fetch(|_| Msg::Variant); // Kicks off an async request.
/// ```
pub trait FetchRequest {
    /// The Request Body (if any).
    type RequestBody: Serialize;
    /// The Response Body (if any).
    type ResponseBody: DeserializeOwned;

    /// What format to use for serialization and deserialization.
    ///
    /// Ideally default to serde_json::Deserializer once
    /// https://github.com/rust-lang/rust/issues/29661
    type Format: Format;

    /// The URL of the resource to fetch.
    fn url(&self) -> String;

    /// The HTTP method and body (if any) to be used in constructing the request.
    fn method(&self) -> MethodBody<Self::RequestBody>;

    /// The headers to attach to the request .
    fn headers(&self) -> Vec<(String, String)>;

    /// Use CORS for the request. By default, it will not.
    fn use_cors(&self) -> bool {
        false
    }
}

pub fn create_request<T: FetchRequest>(request: &T) -> Result<Request, FetchError> {
    let method = request.method();
    let headers = request.headers();
    let headers = JsValue::from_serde(&headers).expect("Convert Headers to Tuple");

    // configure options for the request
    let mut opts = RequestInit::new();
    opts.method(method.as_method());
    opts.body(method.as_body::<T::Format>()?.as_ref());
    opts.headers(&headers);

    // TODO, see if there are more options that can be specified.
    if request.use_cors() {
        opts.mode(RequestMode::Cors);
    }

    // Create the request
    // TODO make this a Rust value instead.
    Request::new_with_str_and_init(&request.url(), &opts).map_err(FetchError::CouldNotCreateRequest)
}

/// Fetch a resource, returning a result of the expected response,
/// or an error indicating what went wrong.
pub async fn fetch_resource<T: FetchRequest>(
    request: Result<Request, FetchError>,
    _req_type: PhantomData<T>,
) -> Result<T::ResponseBody, FetchError> {
    let request = request?;
    // Send the request, resolving it to a response.
    let window: Window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| FetchError::CouldNotCreateFetchFuture)?;
    debug_assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Process the response
    let text = JsFuture::from(resp.text().map_err(|_| FetchError::TextNotAvailable)?)
        .await
        .map_err(|_| FetchError::TextNotAvailable)?;

    let text_string = text.as_string().unwrap();

    // If the response isn't ok, then return an error without trying to deserialize.
    if !resp.ok() {
        return Err(FetchError::ResponseError {
            status_code: resp.status(),
            response_body: text_string,
        });
    }

    let deserialized =
        <T::Format>::deserialize(&text_string).ok_or_else(|| FetchError::DeserializeError {
            error: "".to_string(),
            content: text_string,
        })?;

    Ok(deserialized)
}
