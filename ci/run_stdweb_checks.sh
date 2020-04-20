#!/usr/bin/env bash
echo "$(rustup default)" | grep -q "1.39.0"
emscripten_supported=$?
set -euxo pipefail # https://vaneyckt.io/posts/safer_bash_scripts_with_set_euxo_pipefail/

pushd stdweb-examples
cargo fmt --all -- --check
cargo clippy --all -- --deny=warnings
cargo check --all --target wasm32-unknown-unknown

# webgl_stdweb doesn't play nice with wasm-bindgen
(cd webgl && cargo web check --target wasm32-unknown-unknown)

if [ "$emscripten_supported" == "0" ]; then
  # TODO - Emscripten builds are broken on rustc > 1.39.0
  cargo check --all --target asmjs-unknown-emscripten
  cargo check --all --target wasm32-unknown-emscripten
fi
popd
