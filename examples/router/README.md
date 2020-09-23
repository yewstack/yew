# Router Example

[![Demo](https://img.shields.io/website?label=demo&url=https%3A%2F%2Fexamples.yew.rs%2Frouter)](https://examples.yew.rs/router)

A blog all about yew.
The best way to figure out what this example is about is to just open it up.
It's mobile friendly too!

## Running

While not strictly necessary, this example should be built in release mode:

```bash
trunk serve --release
```

Content generation can take up quite a bit of time in debug builds.

## Concepts

This example involves many different parts, here are just the Yew specific things:

- Uses [`yew-router`] to render and switch between multiple pages.
- Uses [`IntervalService`] for the [`ProgressDelay`](src/components/progress_delay.rs) component.

## Improvements

- Use a special image component which shows a progress bar until the image is loaded.
- Scroll back to the top after switching route
- Run content generation in a dedicated web worker
- Use longer Markov chains to achieve more coherent results
- Make images deterministic (the same seed should produce the same images)
- Show posts by the author on their page
  (this is currently impossible because we need to find post seeds which in turn generate the author's seed)
- Show other posts at the end of a post ("continue reading")
- Home (`/`) should include links to the post list and the author introduction
- Detect sub-path from `--public-url` value passed to Trunk. See: thedodd/trunk#51

[`intervalservice`]: https://docs.rs/yew/latest/yew/services/struct.IntervalService.html
[`yew-router`]: https://docs.rs/yew-router/latest/yew_router/
