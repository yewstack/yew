# Game of Life Example

[![Demo](https://img.shields.io/website?label=demo&url=https%3A%2F%2Fexamples.yew.rs%2Fgame_of_life)](https://examples.yew.rs/game_of_life)

This example boasts a complete implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway's_Game_of_Life).
You can manually toggle cells by clicking on them or create a random layout by pressing the "Random" button.

## Running

This example is quite resource intensive; it's recommended that you only use it with the `--release` flag:

```bash
trunk serve --release
```

## Concepts

Uses `IntervalService` to automatically step the simulation.
Logs to the console using the [`log`](https://crates.io/crates/log) crate with the [`wasm_logger`](https://crates.io/crates/wasm-logger) adapter.
