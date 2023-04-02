# WebGL Example

[![Demo](https://img.shields.io/website?label=demo&url=https%3A%2F%2Fexamples.yew.rs%2Fwebgl)](https://examples.yew.rs/webgl)

This is a simple demo using WebGL with Yew to initialize the GL context, create
a render loop, and draw to the canvas with basic shaders using `web-sys`.

## Concepts

- Accessing a DOM element using [`NodeRef`](https://yew.rs/docs/concepts/components/refs/).
- Using Javascript APIs with `web-sys`.

## Improvements

- Use a much more flashy shader

## Running

Run this application with the trunk development server:

```bash
trunk serve --open
```