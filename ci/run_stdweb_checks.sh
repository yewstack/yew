#!/usr/bin/env bash
echo "$(rustup default)" | grep -q "1.39.0"
emscripten_supported=$?
set -euxo pipefail # https://vaneyckt.io/posts/safer_bash_scripts_with_set_euxo_pipefail/

pushd yew-stdweb
cargo fmt --all -- --check
cargo clippy --all -- --deny=warnings
cargo web check --all --target wasm32-unknown-unknown
if [ "$emscripten_supported" == "0" ]; then
  # TODO - Emscripten builds are broken on rustc > 1.39.0
  cargo web check --all --target asmjs-unknown-emscripten
  cargo web check --all --target wasm32-unknown-emscripten
fi
popd
