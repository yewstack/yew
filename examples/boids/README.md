# Boids Example

[![Demo](https://img.shields.io/website?label=demo&url=https%3A%2F%2Fexamples.yew.rs%2Fboids)](https://examples.yew.rs/boids)

A version of [Boids](https://en.wikipedia.org/wiki/Boids) implemented in Yew.

This example doesn't make use of a [Canvas](https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API),
instead, each boid has its own element demonstrating the performance of Yew's virtual DOM.

## Running

You should run this example with the `--release` flag:

```bash
trunk serve --release
```

## Concepts

The example uses [`gloo::timers`](https://docs.rs/gloo-timers/latest/gloo_timers/) implementation of `setInterval` to drive the Yew game loop.

## Improvements

- Add the possibility to switch the behaviour from flocking to scattering by inverting the cohesion rule so that boids avoid each other.
  This should also invert the color adaption to restore some variety.
- Add keyboard shortcuts for the actions.
- Make it possible to hide the settings panel entirely
- Bigger boids should accelerate slower than smaller ones
- Share settings by encoding them into the URL
- Resize the boids when "Spacing" is changed.
  The setting should then also be renamed to something like "Size".
