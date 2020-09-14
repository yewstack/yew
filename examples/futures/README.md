# Futures Example

Fetches Yew's README file and renders it to the page.

## Concepts

This example shows off how to make asynchronous fetch requests using `web-sys` and Yew's futures support.
It also contains a Markdown renderer which manually creates `Html` without using the `html!` macro.

## Improvements

- `send_future` should be replaced with functionality provided by the `yewtil` crate.
- Markdown rendering code should be cleaned up.
- Should make use of CSS to style the output.
- This example could use a better name.
- Since this features a Markdown renderer it should be possible to render more than just one document.
