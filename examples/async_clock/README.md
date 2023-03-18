# Asynchronous coding in Yew

An example of using asynchronous tasks in a component. This example creates a clock in the background and
continuously awaits clock-ticks. When the clock updates the new time is sent to the UI component to display.
In parallel it fetches online jokes to make the clock more entertaining to watch.

Its main purpose is to demonstrate various ways of using async code in a yew component. It uses the following async
features:
- send_future
- send_stream
- spawn_local
- mpsc::unbounded channels

## Running

Run this application with the trunk development server:

```bash
trunk serve --open
```