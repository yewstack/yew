// # Implementation Note:
//
// This example is also used to demonstrate SSR hydration.
// It is important to follow the following rules when updating this example:
//
// - Do not use usize for randomised contents.
//
//   usize differs in memory size in 32-bit and 64-bit targets (wasm32 is a 32-bit target family.)
//   and would lead to a different value even if the Rng at the same state.
//
// - Do not swap StdRng for SmallRng.
//
//   SmallRng uses different algorithms depending on the platform.
//   Hence, it may not yield the same value on the client and server side.

mod app;
mod components;
mod content;
mod generator;
mod pages;

pub use app::*;
pub use content::*;
pub use generator::*;
