#!/usr/bin/env bash
echo "$(rustup default)" | grep -q "1.39.0"
emscripten_supported=$?
set -euxo pipefail # https://vaneyckt.io/posts/safer_bash_scripts_with_set_euxo_pipefail/

# Showcase includes all other examples
cd examples/showcase

# TODO Can't build some demos with release, need fix

if [ "$emscripten_supported" == "0" ]; then
  # TODO - Emscripten builds are broken on rustc > 1.39.0
  cargo web build --target asmjs-unknown-emscripten
  cargo web build --target wasm32-unknown-emscripten
fi

# TODO showcase doesn't support wasm-bindgen yet
cargo web build --target wasm32-unknown-unknown

# Reset cwd
cd ../..
