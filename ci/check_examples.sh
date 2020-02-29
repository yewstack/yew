#!/usr/bin/env bash
echo "$(rustup default)" | grep -q "1.39.0"
emscripten_supported=$?
set -euxo pipefail # https://vaneyckt.io/posts/safer_bash_scripts_with_set_euxo_pipefail/

# Showcase includes all other examples
cd examples/showcase

# TODO Can't build some demos with release, need fix

if [ "$emscripten_supported" == "0" ]; then
  # TODO - Emscripten builds are broken on rustc > 1.39.0
  cargo web build --target asmjs-unknown-emscripten --features std_web
  cargo web build --target wasm32-unknown-emscripten --features std_web
fi

cargo web build --target wasm32-unknown-unknown --features std_web
cargo build --target wasm32-unknown-unknown --features web_sys

# Reset cwd
cd ../..
