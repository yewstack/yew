#!/usr/bin/env bash
set -euxo pipefail # https://vaneyckt.io/posts/safer_bash_scripts_with_set_euxo_pipefail/

# Showcase includes all other examples
cd examples/showcase
# TODO Can't build some demos with release, need fix
cargo web build --target asmjs-unknown-emscripten
cargo web build --target wasm32-unknown-emscripten
# TODO showcase doesn't support wasm-bindgen yet
cargo web build --target wasm32-unknown-unknown
