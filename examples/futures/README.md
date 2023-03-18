# Futures Example

[![Demo](https://img.shields.io/website?label=demo&url=https%3A%2F%2Fexamples.yew.rs%2Ffutures)](https://examples.yew.rs/futures)

Fetches Yew's README file and renders it to the page.

## Concepts

This example shows off how to make asynchronous fetch requests using `web-sys` and Yew's futures support.
It makes use of yewtil's [`LinkFuture`] to easily send messages asynchronously.
It also contains a Markdown renderer which manually creates `Html` without using the `html!` macro.

## Improvements

- Markdown rendering code should be cleaned up.
- Should make use of CSS to style the output.
- This example could use a better name.
- Since this features a Markdown renderer it should be possible to render more than just one document.

[`linkfuture`]: https://docs.rs/yewtil/latest/yewtil/future/trait.LinkFuture.html

## Running

Run this application with the trunk development server:

```bash
trunk serve --open
```