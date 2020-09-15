# Game of Life Example

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
