#[cfg(feature = "wasm_test")]
use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
use yew::callback::Callback;
use yew::format::Nothing;
use yew::services::fetch::{self, FetchOptions, Response};
use yew::services::FetchService;
use yew::services::Task;

#[cfg(feature = "wasm_test")]
wasm_bindgen_test_configure!(run_in_browser);

#[test]
fn fetch_redirect_follow() {
  let request = fetch::Request::get("https://httpbin.org/relative-redirect/1")
    .body(Nothing)
    .unwrap();
  let options = FetchOptions {
    redirect: Some(fetch::Redirect::Follow),
    ..FetchOptions::default()
  };
  let callback = Callback::from(move |resp: Response<Result<String, failure::Error>>| {
    println!("{:#?}", resp);
  });
  let task = FetchService::new().fetch_with_options(request, options, callback);
  while task.is_active() {}
}
