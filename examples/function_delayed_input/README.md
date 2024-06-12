# Delayed Input Processing Example

[![Demo](https://img.shields.io/website?label=demo&url=https%3A%2F%2Fexamples.yew.rs%2Ffunction_delayed_input)](https://examples.yew.rs/function_delayed_input)

This is a demonstration of how to create an input form with delayed input processing.

A typical use case is to send user input to the backend only when they have stopped typing, rather than on every keystroke.

## Concepts
- Uses [`gloo-timers`](https://crates.io/crates/gloo-timers) to delay the processing

## Running

Run this application with the trunk development server:

```bash
trunk serve --open
```
