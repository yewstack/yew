use wasm_bindgen::JsValue;

/// A representation of an error that may occur when making a fetch request.
#[derive(Debug, PartialEq, Clone)]
pub enum FetchError {
    /// The response could not be deserialized.
    DeserializeError { error: String, content: String },
    /// The response had an error code.
    ResponseError {
        status_code: u16,
        response_body: String,
    },
    /// Text was not available on the response.
    // TODO, this might get thrown in unexpected circumstances.
    TextNotAvailable,
    /// The Fetch Future could not be created due to a misconfiguration.
    CouldNotCreateFetchFuture,
    /// The request could cont be created due to a misconfiguration.
    CouldNotCreateRequest(JsValue), // TODO, convert this to a string or more structured error - implement Hash on this and related structs.
    /// Could not serialize the request body.
    CouldNotSerializeRequestBody,
}

impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FetchError::DeserializeError { error, content } => f.write_str(&format!(
                "Could not deserialize a successful request. With error: {}, and content: {}",
                error, content
            )),
            FetchError::ResponseError {
                status_code,
                response_body,
            } => f.write_str(&format!(
                "The server returned a response with code: {}, and body: {}",
                status_code, response_body
            )),
            FetchError::TextNotAvailable => {
                f.write_str("The text could not be extracted from the response.")
            }
            FetchError::CouldNotCreateFetchFuture => {
                f.write_str("Could not create a fetch future.")
            }
            FetchError::CouldNotCreateRequest(_) => {
                f.write_str("Could not create a fetch request.")
            }
            FetchError::CouldNotSerializeRequestBody => {
                f.write_str("Could not serialize the body in the fetch request.")
            }
        }
    }
}

impl std::error::Error for FetchError {}
