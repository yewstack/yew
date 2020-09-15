# CCXT and Gravatar Example

I honestly have a hard time describing what this example does...
You can either list cryptocurrency exchanges powered by [CCXT] or display some data from a hard-coded [Gravatar] profile.
How do these thing fit together? I don't know.

## Concepts

- Uses the [`FetchService`] to retrieve the Gravatar profile.
- Interop with the CCXT js library using `wasm-bindgen`.

## Improvements

This example can be seen as a mix of [dashboard](../dashboard) (fetching data) and [js_callback](../js_callback) (js interop).
It lacks a distinct purpose.
If you have something in mind that can use Gravatar, CCXT, or even both, feel free to reshape this example however you wish.

- Handle Gravatar API errors properly
- Create proper js bindings for CCTX instead of using reflection
- Improve the presentation using CSS

[ccxt]: https://github.com/ccxt/ccxt
[gravatar]: https://gravatar.com
[`fetchservice`]: https://docs.rs/yew/latest/yew/services/fetch/struct.FetchService.html
